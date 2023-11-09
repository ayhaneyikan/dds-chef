mod execution_control_service;
use std::{env, thread::sleep, time::Duration};

use common::recipe::Recipe;
use execution_control_service::HeadChefService;

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
    let mut p: HeadChefService;

    // attempt to read in recipe file and initialize control service
    if let Some(file_name) = retreive_filename() {
        // attempting to read in provided recipe file
        match Recipe::from_file(&file_name) {
            Ok(recipe) => p = HeadChefService::new(recipe),
            Err(e) => {
                println!("Error reading recipe: {}", e);
                return;
            }
        }
    } else {
        // filename not passed in successfully
        println!("Usage: driver -f <recipe-filename>");
        return;
    }

    // initialization delay
    sleep(Duration::from_secs(5));

    println!("Beginning chef-ing");
    while !p.check_completed() {
        p.cycle();
    }

    println!("Chef-ing complete!");
}
