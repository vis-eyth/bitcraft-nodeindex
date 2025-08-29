use bindings::{sdk::{DbContext, Error}, region::*};
use tracing::info;

#[derive(Debug)]
pub enum Query {
    ENEMY,
    RESOURCE(i32),
}

impl Query {
    fn query(&self) -> Vec<String> {
        match self {
            Query::ENEMY => vec![
                concat!(
                    "SELECT mob.* FROM enemy_state mob",
                    " JOIN mobile_entity_state loc ON mob.entity_id = loc.entity_id;").to_string(),
                concat!(
                    "SELECT loc.* FROM mobile_entity_state loc",
                    " JOIN enemy_state mob ON loc.entity_id = mob.entity_id;").to_string(),
            ],
            Query::RESOURCE(id) => vec![
                format!(concat!(
                    "SELECT res.* FROM resource_state res",
                    " JOIN location_state loc ON res.entity_id = loc.entity_id",
                    " WHERE res.resource_id = {};"), id),
                format!(concat!(
                    "SELECT loc.* FROM location_state loc",
                    " JOIN resource_state res ON loc.entity_id = res.entity_id",
                    " WHERE res.resource_id = {};"), id),
            ]
        }
    }
}

pub struct QueueSub {
    queries: Vec<Query>,

    on_success: Option<Box<dyn FnOnce() + Send>>,
    on_error:   Option<fn(&ErrorContext, Error)>,
}

impl QueueSub {
    pub fn with(queries: Vec<Query>) -> Self {
        QueueSub {
            queries,

            on_success: None,
            on_error:   None,
        }
    }

    pub fn on_success(mut self, on_success: impl FnOnce() -> () + Send + 'static) -> Self {
        self.on_success = Some(Box::new(on_success)); self
    }
    pub fn on_error(mut self, on_error: fn(&ErrorContext, Error)) -> Self {
        self.on_error = Some(on_error); self
    }


    fn next<C>(self, idx: usize, ctx: &C)
    where C: DbContext<
        DbView=<DbConnection as DbContext>::DbView,
        Reducers=<DbConnection as DbContext>::Reducers,
        SetReducerFlags=<DbConnection as DbContext>::SetReducerFlags,
        SubscriptionBuilder=<DbConnection as DbContext>::SubscriptionBuilder>
    {
        if self.queries.len() <= idx {
            info!("all subscriptions active!");
            if let Some(on_success) = self.on_success { on_success() }
            return;
        }

        let sub = &self.queries[idx];
        let query = sub.query();
        info!("[{:>3}/{:>3}] subscribing to {:?}", idx+1, self.queries.len(), sub);

        ctx.subscription_builder()
            .on_error(move |ctx, e| {
                if let Some(on_error) = self.on_error { on_error(&ctx, e) }
            })
            .on_applied(move |ctx| {
                self.next(idx+1, ctx)
            })
            .subscribe(query);
    }
}

pub trait WithQueueSub {
    fn subscribe(&self, sub: QueueSub);
}

impl WithQueueSub for DbConnection {
    fn subscribe(&self, sub: QueueSub) {
        sub.next(0, self);
    }
}