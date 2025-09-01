use std::sync::Arc;
use bindings::region::DbUpdate;
use hashbrown::{HashMap, HashSet};
use tokio::sync::mpsc::UnboundedReceiver;
use crate::config::{AppState, DataState};

struct Update {
    insert: HashMap<u64, [i32; 2]>,
    delete: HashSet<u64>,
}
impl Update {
    fn new() -> Self { Self { insert: HashMap::new(), delete: HashSet::new() } }
    fn additional(&self) -> usize {
        let insert = self.insert.len();
        let delete = self.delete.len();

        if insert > delete { insert - delete } else { 0 }
    }
}

pub async fn consume(mut rx: UnboundedReceiver<DbUpdate>, state: Arc<AppState>) {
    // updates is drained during each apply, so free to re-use.
    let mut updates = HashMap::new();
    // enemy_state needs to be kept across iterations since enemy locations update.
    let mut enemy_state = HashMap::new();

    while let Some(update) = rx.recv().await {
        // location_state should always be inserted in the batch the corresponding entity
        // is added, so the map can be cleared across iterations.
        let mut location_state = HashMap::new();
        for e in update.location_state.inserts {
            location_state.insert(e.row.entity_id, [e.row.x, e.row.z]);
        }

        // as resources cannot move all inserts and deletes are handled via resource_state.
        for e in update.resource_state.deletes {
            updates.entry(e.row.resource_id)
                .or_insert_with(Update::new)
                .delete
                .insert(e.row.entity_id);
        }
        for e in update.resource_state.inserts {
            let loc = location_state.get(&e.row.entity_id).unwrap().clone();
            updates.entry(e.row.resource_id)
                .or_insert_with(Update::new)
                .insert
                .insert(e.row.entity_id, loc);
        }

        for (res_id, updates) in updates.drain() {
            let Some(data) = state.resource.get(&res_id) else { continue };
            let mut data = data.write().await;

            if data.state == DataState::STALE {
                data.nodes.clear();
                data.state = DataState::ACTIVE;
            }

            data.nodes.reserve(updates.additional());
            for e_id in updates.delete { data.nodes.remove(&e_id); }
            for (e_id, loc) in updates.insert { data.nodes.insert(e_id, loc); }
        }

        // build index for enemy_type for entity_id
        // deletes are handled via enemy_state, but inserts are
        // handled via mobile_entity_state, as they also handle moves
        for e in update.enemy_state.deletes {
            enemy_state.remove(&e.row.entity_id);

            updates.entry(e.row.enemy_type as i32)
                .or_insert_with(Update::new)
                .delete
                .insert(e.row.entity_id);
        }
        for e in update.enemy_state.inserts {
            enemy_state.insert(e.row.entity_id, e.row.enemy_type as i32);
        }
        for e in update.mobile_entity_state.inserts {
            let mob_id = enemy_state.get(&e.row.entity_id).unwrap().clone();
            updates.entry(mob_id)
                .or_insert(Update::new())
                .insert
                .insert(e.row.entity_id, [e.row.location_x, e.row.location_z]);
        }

        for (mob_id, updates) in updates.drain() {
            let Some(data) = state.enemy.get(&mob_id) else { continue };
            let mut data = data.write().await;

            if data.state == DataState::STALE {
                data.nodes.clear();
                data.state = DataState::ACTIVE;
            }

            data.nodes.reserve(updates.additional());
            for e_id in updates.delete { data.nodes.remove(&e_id); }
            for (e_id, loc) in updates.insert { data.nodes.insert(e_id, loc); }
        }
    }
}