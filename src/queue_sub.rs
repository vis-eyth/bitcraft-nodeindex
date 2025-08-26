use std::collections::VecDeque;
use bindings::{sdk::{DbContext, Error}, region::*};

enum QueueEvent {
    GROUP(String),
    QUERY(Box<dyn FnOnce() -> Vec<String> + Send>),
}

pub struct QueueSub {
    queue: VecDeque<QueueEvent>,

    on_success: Option<fn()>,
    on_error:   Option<fn(&ErrorContext, Error)>,
    on_group:   Option<fn(&str)>,
    on_tick:    Option<fn()>,
}

impl QueueSub {
    pub fn new() -> Self {
        QueueSub {
            queue: VecDeque::new(),

            on_success: None,
            on_error:   None,
            on_group:   None,
            on_tick:    None,
        }
    }

    pub fn on_success(mut self, on_success: fn()) -> Self {
        self.on_success = Some(on_success); self
    }
    pub fn on_error(mut self, on_error: fn(&ErrorContext, Error)) -> Self {
        self.on_error = Some(on_error); self
    }
    pub fn on_group(mut self, on_group: fn(&str)) -> Self {
        self.on_group = Some(on_group); self
    }
    pub fn on_tick(mut self, on_tick: fn()) -> Self{
        self.on_tick = Some(on_tick); self
    }


    pub fn push_group(&mut self, group: String) {
        self.queue.push_back(QueueEvent::GROUP(group))
    }
    pub fn push_query(&mut self, query: impl FnOnce() -> Vec<String> + Send + 'static) {
        self.queue.push_back(QueueEvent::QUERY(Box::new(query)))
    }


    fn next(mut self, ctx: Subscribable) {
        loop {
            match self.queue.pop_front() {
                None => {
                    if let Some(on_success) = self.on_success { on_success() }
                    break;
                }
                Some(QueueEvent::GROUP(group)) => {
                    if let Some(on_group) = self.on_group { on_group(&group) }
                }
                Some(QueueEvent::QUERY(query)) => {
                    match ctx {
                        Subscribable::CON(con) => con.subscription_builder(),
                        Subscribable::CTX(ctx) => ctx.subscription_builder(),

                    }.on_error(move |ctx, e| {
                        if let Some(on_error) = self.on_error { on_error(&ctx, e) }

                    }).on_applied(|ctx| {
                        if let Some(on_tick) = self.on_tick { on_tick() }
                        self.next(Subscribable::CTX(ctx))

                    }).subscribe(query());
                    break;
                }
            }
        }
    }
}

enum Subscribable<'a> { CON(&'a DbConnection), CTX(&'a SubscriptionEventContext) }

pub trait WithQueueSub {
    fn subscribe(&self, sub: QueueSub);
}

impl WithQueueSub for DbConnection {
    fn subscribe(&self, sub: QueueSub) {
        sub.next(Subscribable::CON(self));
    }
}