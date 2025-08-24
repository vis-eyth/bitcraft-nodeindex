use std::collections::HashMap;
use bindings::region::*;

mod resource;
use resource::{ResourceSubscription};

mod glue;
use glue::{Config, Configurable, with_channel};

use spacetimedb_sdk::{DbContext, Table};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

enum Message {
    Disconnect,
    Insert { id: u64, res: i32, x: i32, z: i32 },
    Delete { id: u64, res: i32 },
}

impl Message {
    pub fn insert(res: &ResourceState, loc: &LocationState) -> Self {
        Self::Insert { id: res.entity_id, res: res.resource_id, x: loc.x, z: loc.z }
    }

    pub fn delete(res: &ResourceState) -> Self {
        Self::Delete { id: res.entity_id, res: res.resource_id }
    }
}

#[tokio::main]
async fn main() {
    let config = Config::from("config.json").expect("failed to load config.json");

    if config.is_empty() {
        eprintln!("please fill out the configuration file (config.json)!");
        return;
    }

    let (tx, rx) = unbounded_channel::<Message>();
    let con = DbConnection::builder()
        .configure(&config)
        .on_connect(|ctx, _, _| {
            eprintln!("connected!");
            ctx.subscription_builder().all_resources(
                |_, err| eprintln!("subscription error: {:?}", err),
                |_| println!("active!"));
        })
        .on_disconnect(|_, _| eprintln!("disconnected!"))
        .build()
        .unwrap();

    con.db.resource_state().on_insert(with_channel(tx.clone(), on_insert));
    con.db.resource_state().on_delete(with_channel(tx.clone(), on_delete));

    let mut producer = Box::pin(con.run_async());
    let consumer = tokio::spawn(consume(rx));

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            con.disconnect().unwrap();
            producer.await.unwrap();
            tx.send(Message::Disconnect).unwrap();
            consumer.await.unwrap();
        },
        _ = &mut producer => {
            println!("server disconnect!");
            tx.send(Message::Disconnect).unwrap();
            consumer.await.unwrap();
        },
    }
}

fn on_insert(ctx: &EventContext, row: &ResourceState, tx: &UnboundedSender<Message>) {
    let loc = ctx.db.location_state()
        .entity_id()
        .find(&row.entity_id);

    if let Some(loc) = loc {
        tx.send(Message::insert(row, &loc)).unwrap()
    } else {
        eprintln!("no location found for resource: {}", row.entity_id);
    }
}

fn on_delete(_: &EventContext, row: &ResourceState, tx: &UnboundedSender<Message>) {
    tx.send(Message::delete(row)).unwrap()
}

async fn consume(mut rx: UnboundedReceiver<Message>) {
    let mut map: HashMap<i32, HashMap<u64, (i32, i32)>> = HashMap::new();

    while let Some(msg) = rx.recv().await {
        if let Message::Disconnect = &msg {
            println!("resource counts:");
            for (res, nodes) in &map {
                println!("{:>11}: {} entries", res, nodes.len());
            }
            break
        }

        if let Message::Insert{ id, res, x, z } = &msg {
            map.entry(*res)
                .or_insert_with(HashMap::new)
                .insert(*id, (*x, *z));
        }

        if let Message::Delete { id, res } = &msg {
            map.get_mut(res)
                .unwrap()
                .remove(id);
        }
    }
}