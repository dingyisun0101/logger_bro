/// ==============================================================================
/// examples/dummy_project_2/main.rs
/// End-to-end dummy runner where progress is read from outside the simulator.
/// ==============================================================================

mod dummy_task;
mod dummy_task_group;

use logger_bro::TaskGroup;
use dummy_task_group::DummyTaskGroup;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let task_group = DummyTaskGroup::new(4);
    task_group.run(20)
}
