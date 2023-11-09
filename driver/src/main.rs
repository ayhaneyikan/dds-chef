mod execution_control_service;
use std::{env, thread::sleep, time::Duration};

use common::recipe::Recipe;
use execution_control_service::HeadChefControlService;

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
    let mut p: HeadChefControlService;

    // attempt to read in recipe file and initialize control service
    if let Some(file_name) = retreive_filename() {
        // attempting to read in provided recipe file
        match Recipe::from_file(&file_name) {
            Ok(recipe) => p = HeadChefControlService::new(recipe),
            Err(e) => {
                println!("Error reading recipe: {}", e);
                return;
            }
        }
    } else {
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
