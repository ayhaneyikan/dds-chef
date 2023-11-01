mod execution_control_service;
use std::{thread::sleep, time::Duration};

use execution_control_service::ExecutionControlService;

fn main() {
    // initialize publisher service
    let mut p = ExecutionControlService::new();

    // initialization delay
    sleep(Duration::from_secs(5));

    println!("Running Publisher");
    while !p.check_completed() {
        p.cycle();
    }

    println!("Publisher completed running successfully!");
}
