use std::sync::Arc;
use bindings::region::DbUpdate;
use intmap::IntMap;
use tokio::sync::mpsc::UnboundedReceiver;
use crate::config::AppState;

struct Update {
    insert: IntMap<u64, [i32; 2]>,
    delete: Vec<u64>,
}
impl Update {
    fn new() -> Self { Self { insert: IntMap::new(), delete: Vec::new() } }
}

pub async fn consume(mut rx: UnboundedReceiver<DbUpdate>, state: Arc<AppState>) {
    let mut rev_enemy_state = IntMap::new();

    while let Some(update) = rx.recv().await {
        let mut location_state = IntMap::new();
        let mut updates = IntMap::new();

        // all resources should arrive with location_state inserts
        // deletes are handled via delete on resource_state, no moves should happen here.
        for e in update.location_state.inserts {
            location_state.insert(e.row.entity_id, [e.row.x, e.row.z]);
        }
        for e in update.resource_state.deletes {
            updates.entry(e.row.resource_id)
                .or_insert_with(Update::new)
                .delete
                .push(e.row.entity_id);
        }
        for e in update.resource_state.inserts {
            updates.entry(e.row.resource_id)
                .or_insert_with(Update::new)
                .insert
                .insert(e.row.entity_id, location_state.get(e.row.entity_id).unwrap().clone());
        }

        for (res_id, updates) in updates.drain() {
            let Some(map) = state.resource.get(res_id) else { continue };
            let mut map = map.nodes.write().await;

            for e_id in updates.delete { map.remove(e_id); }
            for (e_id, loc) in updates.insert { map.insert(e_id, loc); }
        }

        let mut updates = IntMap::new();

        // build reverse index for enemy_type for entity_id
        // deletes are handled via enemy_state, but inserts are
        // handled via mobile_entity_state, as they also handle moves
        for e in update.enemy_state.deletes {
            rev_enemy_state.remove(e.row.entity_id);

            updates.entry(e.row.enemy_type as i32)
                .or_insert_with(Update::new)
                .delete
                .push(e.row.entity_id);
        }
        for e in update.enemy_state.inserts {
            rev_enemy_state.insert(e.row.entity_id, e.row.enemy_type as i32);
        }
        for e in update.mobile_entity_state.inserts {
            updates.entry(rev_enemy_state.get(e.row.entity_id).unwrap().clone())
                .or_insert(Update::new())
                .insert
                .insert(e.row.entity_id, [e.row.location_x, e.row.location_z]);
        }

        for (mob_id, updates) in updates.drain() {
            let Some(map) = state.enemy.get(mob_id) else { continue };
            let mut map = map.nodes.write().await;

            for e_id in updates.delete { map.remove(e_id); }
            for (e_id, loc) in updates.insert { map.insert(e_id, loc); }
        }
    }
}