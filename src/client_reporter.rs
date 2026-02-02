/// ==============================================================================
/// src/client_reporter.rs
/// Client-side reporting API for sending progress updates to the store.
/// ==============================================================================

use std::sync::mpsc::Sender;

use std::time::Instant;

use crate::{ClientState, TaskId, TaskStatus};

/// Errors that can occur when sending updates from a client thread.
#[derive(Debug)]
pub enum ReportError {
    /// The receiver/store side has been dropped, so updates can no longer be sent.
    Closed,
}

/// Lightweight client-side handle for sending updates into the store.
///
/// This type is intended to be cloned and moved across threads without
/// carrying any store internals.
#[derive(Clone)]
pub struct ClientReporter {
    tx: Sender<ClientState>,
}

/// Handle for a single client/task instance.
///
/// The handle owns the task identity and start time, and emits partial
/// updates using those fields as a stable base.
#[derive(Clone)]
pub struct ClientHandle {
    reporter: ClientReporter,
    id: TaskId,
    start_time: Instant,
}

impl ClientReporter {
    /// Create a reporter from a sender that feeds the store.
    pub fn new(tx: Sender<ClientState>) -> Self {
        Self { tx }
    }

    /// Send a raw `ClientState` update to the store.
    ///
    /// This is the lowest-level API; most users should prefer `start`
    /// and the `ClientHandle` methods.
    pub fn report(&self, state: ClientState) -> Result<(), ReportError> {
        self.tx.send(state).map_err(|_| ReportError::Closed)
    }

    /// Start a new client/task and return a handle for future updates.
    ///
    /// This sends an initial full state (including label and optional total)
    /// and returns a handle that emits partial updates afterwards.
    pub fn start(
        &self,
        label: impl Into<String>,
        total: Option<u64>,
    ) -> Result<ClientHandle, ReportError> {
        let state = ClientState::new(label, total);
        let handle = ClientHandle {
            reporter: self.clone(),
            id: state.id,
            start_time: state.start_time,
        };
        self.report(state)?;
        Ok(handle)
    }
}

impl ClientHandle {
    /// Return the internal identifier for this client/task.
    pub fn id(&self) -> TaskId {
        self.id
    }

    /// Update the display label for this client/task.
    pub fn set_label(&self, label: impl Into<String>) -> Result<(), ReportError> {
        let mut update = self.base_update();
        update.label = Some(label.into());
        self.reporter.report(update)
    }

    /// Update the total units of work for this client/task.
    ///
    /// Use `None` for an indeterminate total.
    pub fn set_total(&self, total: Option<u64>) -> Result<(), ReportError> {
        let mut update = self.base_update();
        update.total = total;
        self.reporter.report(update)
    }

    /// Set the current completed units of work.
    pub fn set_current(&self, current: u64) -> Result<(), ReportError> {
        let mut update = self.base_update();
        update.current = Some(current);
        self.reporter.report(update)
    }

    /// Mark this client/task as completed.
    pub fn complete(&self) -> Result<(), ReportError> {
        self.set_status(TaskStatus::Completed)
    }

    /// Mark this client/task as failed.
    pub fn fail(&self) -> Result<(), ReportError> {
        self.set_status(TaskStatus::Failed)
    }

    /// Mark this client/task as canceled.
    pub fn cancel(&self) -> Result<(), ReportError> {
        self.set_status(TaskStatus::Canceled)
    }

    /// Internal helper to set a status update.
    fn set_status(&self, status: TaskStatus) -> Result<(), ReportError> {
        let mut update = self.base_update();
        update.status = Some(status);
        self.reporter.report(update)
    }

    /// Construct a minimal update payload with identity and timestamps.
    ///
    /// The store will merge this partial state with the existing one.
    fn base_update(&self) -> ClientState {
        ClientState::partial(self.id, self.start_time, Instant::now())
    }
}
