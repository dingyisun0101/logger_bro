/// ==============================================================================
/// src/client_store.rs
/// In-memory store for the latest client states.
/// ==============================================================================

use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver};

use crate::{ClientReporter, ClientState, TaskId};

/// In-memory store of the latest client states keyed by `TaskId`.
///
/// This type is intended to live on the UI/runtime side and be fed by
/// a `ClientReporter` running in simulation threads.
#[derive(Debug)]
pub struct ClientStore {
    rx: Receiver<ClientState>,
    clients: HashMap<TaskId, ClientState>,
}

impl ClientStore {
    /// Create a new store and a paired reporter that feeds it.
    pub fn new() -> (ClientReporter, Self) {
        let (tx, rx) = mpsc::channel();
        let reporter = ClientReporter::new(tx);
        let store = Self {
            rx,
            clients: HashMap::new(),
        };
        (reporter, store)
    }

    /// Drain all pending updates and merge them into the store.
    ///
    /// This is non-blocking and processes all currently queued updates.
    pub fn drain(&mut self) {
        for state in self.rx.try_iter() {
            self.clients
                .entry(state.id)
                .and_modify(|existing| merge_state(existing, &state))
                .or_insert(state);
        }
    }

    /// Return a snapshot of the latest known state for all clients.
    ///
    /// The snapshot is a cloned vector to keep read access independent
    /// from the store's internal mutation.
    pub fn snapshot(&self) -> Vec<ClientState> {
        self.clients.values().cloned().collect()
    }
}

/// Merge a partial update into the existing stored state.
///
/// Fields that are `None` in the update are left unchanged.
fn merge_state(existing: &mut ClientState, update: &ClientState) {
    if let Some(label) = &update.label {
        existing.label = Some(label.clone());
    }
    if let Some(status) = update.status {
        existing.status = Some(status);
    }
    if update.total.is_some() {
        existing.total = update.total;
    }
    if let Some(new_current) = update.current {
        let prev_current = existing.current;
        if prev_current != Some(new_current) {
            existing.last_iter_duration = Some(
                update
                    .last_update
                    .duration_since(existing.last_progress_update),
            );
            existing.last_progress_update = update.last_update;
        }
        existing.current = Some(new_current);
    }
    existing.start_time = update.start_time;
    existing.last_update = update.last_update;
}
