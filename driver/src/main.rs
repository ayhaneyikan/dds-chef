mod execution_control_service;
use std::{thread::sleep, time::Duration, env};

use execution_control_service::ExecutionControlService;

fn main() {
    // read in mission plan file name
    let args: Vec<String> = env::args().collect();

    let mut in_file = None;
    for (i, a) in args.iter().enumerate() {
        if a == "-f" && i + 1 < args.len() {
            in_file = Some(args[i + 1].clone());
            // TODO: perform some check for existance/validity of filename
        }
    }
    // ensure a filename was provided before proceeding
    if in_file.is_none() {
        println!("Usage: driver -f <mission-plan-filename>");
        return;
    }

    // initialize publisher service
    let mut p = ExecutionControlService::new(in_file.unwrap());

    // initialization delay
    sleep(Duration::from_secs(5));

    println!("Running Publisher");
    while !p.check_completed() {
        p.cycle();
    }

    println!("Publisher completed running successfully!");
}
