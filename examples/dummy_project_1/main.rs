/// ==============================================================================
/// examples/dummy_project/main.rs
/// End-to-end dummy runner: cluster -> store -> runtime.
/// ==============================================================================

mod dummy_cluster;
mod dummy_simulator;

use std::thread;

use logger_bro::prelude::client::*;
#[cfg(feature = "tui")]
use logger_bro::prelude::runtime::*;
use dummy_cluster::DummyCluster;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (reporter,  mut store) = ClientStore::new();

    let cluster = DummyCluster::new(4);
    let sim_thread = thread::spawn({
        let reporter = reporter.clone();
        move || {
            cluster.run(20, reporter);
        }
    });

    #[cfg(feature = "tui")]
    {
        let mut runtime = Runtime::new(20);
        runtime.run(&mut store)?;
    }

    let _ = sim_thread.join();
    Ok(())
}
