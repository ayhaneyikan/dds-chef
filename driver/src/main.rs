mod execution_control_service;
use std::{thread::sleep, time::Duration, env};

use execution_control_service::ExecutionControlService;

fn retreive_filename() -> Option<String> {
    // read in mission plan file name
    let args: Vec<String> = env::args().collect();

    let mut in_file = None;
    for (i, a) in args.iter().enumerate() {
        if a == "-f" && i + 1 < args.len() {
            in_file = Some(args[i + 1].clone());
            // TODO: perform some check for existance/validity of filename
        }
    }
    in_file
}

fn main() {
    let mut p: ExecutionControlService;
    if let Some(file_name) = retreive_filename() {
        // initialize publisher service
        p = ExecutionControlService::new(file_name);
    }
    else {
        // filename not passed in successfully
        println!("Usage: driver -f <mission-plan-filename>");
        return;
    }


    // initialization delay
    sleep(Duration::from_secs(5));

    println!("Running Publisher");
    while !p.check_completed() {
        p.cycle();
    }

    println!("Publisher completed running successfully!");
}
