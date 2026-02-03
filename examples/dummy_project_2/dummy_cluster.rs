/// ==============================================================================
/// examples/dummy_project_2/dummy_cluster.rs
/// Dummy cluster where progress is observed externally.
/// ==============================================================================

use std::thread;
use std::time::Duration;

use logger_bro::prelude::client::*;

use super::dummy_simulator::{DummySimulator, new_probe};

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
    /// Progress is read from a shared probe rather than reported by the simulator.
    pub fn run(&self, steps: usize, reporter: ClientReporter) {
        let mut handles = Vec::with_capacity(self.count * 2);
        for i in 0..self.count {
            let reporter = reporter.clone();
            let label = format!("sim-{i}");
            let probe = new_probe();

            let sim_probe = probe.clone();
            let sim_handle = thread::spawn(move || {
                let sim = DummySimulator::new(sim_probe);
                sim.run(steps);
            });

            let observer_handle = thread::spawn(move || {
                let client = reporter.start(label, Some(steps as u64));
                if let Ok(client) = client {
                    let mut last = 0usize;
                    loop {
                        let current = probe.current();
                        if current != last {
                            let _ = client.set_current(current as u64);
                            last = current;
                        }
                        if probe.is_done() && current >= steps {
                            break;
                        }
                        thread::sleep(Duration::from_millis(200));
                    }
                    let _ = client.set_current(steps as u64);
                    let _ = client.complete();
                }
            });

            handles.push(sim_handle);
            handles.push(observer_handle);
        }

        for handle in handles {
            let _ = handle.join();
        }
    }
}
