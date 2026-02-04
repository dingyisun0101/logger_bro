/// ==============================================================================
/// examples/dummy_project/main.rs
/// End-to-end dummy runner using the dummy task group launcher.
/// ==============================================================================
mod dummy_task;
mod dummy_task_group;

use logger_bro::TaskGroup;
use dummy_task_group::DummyTaskGroup;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    DummyTaskGroup::new(4, 20).launch()
}
