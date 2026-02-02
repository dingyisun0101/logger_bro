/// ==============================================================================
/// examples/dummy_project/dummy_cluster.rs
/// Minimal dummy cluster for running simulators in parallel.
/// ==============================================================================

use std::thread;

use logger_bro::prelude::client::*;

use super::dummy_simulator::DummySimulator;

/// A cluster that runs multiple dummy simulators in parallel threads.
#[derive(Debug)]
pub struct DummyCluster {
    /// Number of simulators to spawn.
    pub count: usize,
}

impl DummyCluster {
    /// Create a cluster with `count` simulators.
    pub fn new(count: usize) -> Self {
        Self { count }
    }

    /// Run all simulators in parallel, each for `steps` ticks.
    ///
    /// Each simulator gets its own client handle and reports progress.
    pub fn run(&self, steps: usize, reporter: ClientReporter) {
        let mut handles = Vec::with_capacity(self.count);
        for i in 0..self.count {
            let reporter = reporter.clone();
            let label = format!("sim-{i}");
            handles.push(thread::spawn(move || {
                let mut sim = DummySimulator::new();
                let client = reporter.start(label, Some(steps as u64));
                if let Ok(client) = client {
                    for _ in 0..steps {
                        sim.tick();
                        let _ = client.set_current(sim.step as u64);
                    }
                    let _ = client.complete();
                }
                sim
            }));
        }

        for handle in handles {
            let _ = handle.join();
        }
    }
}
