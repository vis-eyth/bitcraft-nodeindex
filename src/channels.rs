use bindings::sdk::{Table, TableWithPrimaryKey};
use tokio::sync::mpsc::UnboundedSender;

pub trait SendRowEvent<T: Table, M: Send + 'static> {
    fn on_insert_send(
        &self,
        sender: &UnboundedSender<M>,
        callback: impl FnMut(&T::EventContext, &T::Row) -> Option<M> + Send + 'static,
    ) -> T::InsertCallbackId;

    fn on_delete_send(
        &self,
        sender: &UnboundedSender<M>,
        callback: impl FnMut(&T::EventContext, &T::Row) -> Option<M> + Send + 'static,
    ) -> T::DeleteCallbackId;
}

impl<T: Table, M: Send + 'static> SendRowEvent<T, M> for T {
    fn on_insert_send(
        &self,
        sender: &UnboundedSender<M>,
        mut callback: impl FnMut(&T::EventContext, &T::Row) -> Option<M> + Send + 'static
    ) -> T::InsertCallbackId {
        let sender = sender.clone();
        self.on_insert(move |ctx, row| {
            if let Some(m) = callback(&ctx, row) { sender.send(m).unwrap(); }
        })
    }

    fn on_delete_send(
        &self,
        sender: &UnboundedSender<M>,
        mut callback: impl FnMut(&T::EventContext, &T::Row) -> Option<M> + Send + 'static
    ) -> T::DeleteCallbackId {
        let sender = sender.clone();
        self.on_delete(move |ctx, row| {
            if let Some(m) = callback(&ctx, row) { sender.send(m).unwrap(); }
        })
    }
}


pub trait SendRowUpdateEvent<T: TableWithPrimaryKey, M: Send + 'static> {
    fn on_update_send(
        &self,
        sender: &UnboundedSender<M>,
        callback: impl FnMut(&T::EventContext, &T::Row, &T::Row) -> Option<M> + Send + 'static,
    ) -> T::UpdateCallbackId;
}


impl<T: TableWithPrimaryKey, M: Send + 'static> SendRowUpdateEvent<T, M> for T {
    fn on_update_send(
        &self,
        sender: &UnboundedSender<M>,
        mut callback: impl FnMut(&T::EventContext, &T::Row, &T::Row) -> Option<M> + Send + 'static
    ) -> T::UpdateCallbackId {
        let sender = sender.clone();
        self.on_update(move |ctx, old, new| {
            if let Some(m) = callback(&ctx, old, new) { sender.send(m).unwrap(); }
        })
    }
}