/// Convenient re-exports for client-side usage.

pub mod client {
    pub use crate::{
        ClientHandle, ClientReporter, ClientState, ClientStore, ReportError, TaskId, TaskStatus,
    };
}

#[cfg(feature = "tui")]
pub mod runtime {
    pub use crate::Runtime;
}
