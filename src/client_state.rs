/// ==============================================================================
/// src/client_state.rs
/// Core progress state shared across modules.
///
/// Each `ClientState` represents a "client" (what we previously called a task)
/// whose progress we want to track. This type is part of the public API and is
/// intended for client-side consumption (simulators, UIs, and exporters).
/// ==============================================================================


use std::time::Instant;

/// Opaque unique identifier for a client.
/// The identifier used for internal bookkeeping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(pub u64);

impl TaskId {
    pub fn new() -> Self {
        Self(rand::random())
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Running,
    Completed,
    Failed,
    Canceled,
}

#[derive(Debug, Clone)]
pub struct ClientState {
    /// Opaque unique identifier for internal store bookkeeping; never displayed.
    pub id: TaskId,
    /// Human-readable label for display; may be unset until known.
    pub label: Option<String>,
    /// Current lifecycle state of the task; may be unset if unknown.
    pub status: Option<TaskStatus>,
    /// Optional total units of work; `None` for unknown/indeterminate total.
    pub total: Option<u64>,
    /// Current completed units; `None` if not reported yet.
    pub current: Option<u64>,
    /// Monotonic start time of the task, used for elapsed calculations.
    pub start_time: Instant,
    /// Monotonic time of the most recent update to this task.
    pub last_update: Instant,
}

impl ClientState {
    /// Create a new full client state with required identity and timestamps.
    pub fn new(label: impl Into<String>, total: Option<u64>) -> Self {
        let now = Instant::now();
        Self {
            id: TaskId::new(),
            label: Some(label.into()),
            status: Some(TaskStatus::Running),
            total,
            current: Some(0),
            start_time: now,
            last_update: now,
        }
    }

    /// Create a partial update payload for an existing client.
    ///
    /// All user-facing fields are unset; only identity and timestamps are
    /// populated so the store can merge updates correctly.
    pub fn partial(id: TaskId, start_time: Instant, last_update: Instant) -> Self {
        Self {
            id,
            label: None,
            status: None,
            total: None,
            current: None,
            start_time,
            last_update,
        }
    }
}
