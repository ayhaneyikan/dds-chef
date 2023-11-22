mod head_chef_service;
use std::{env, thread::sleep, time::Duration};

use common::recipe::Recipe;
use head_chef_service::HeadChefService;

/// Helper which attempts to retrieve a filename from provided command line args
fn retreive_filename() -> Option<String> {
    // read in command line args
    let args: Vec<String> = env::args().collect();

    let mut in_file = None;
    for (i, a) in args.iter().enumerate() {
        if a == "-f" && i + 1 < args.len() {
            in_file = Some(args[i + 1].clone());
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
            Ok(recipe) => {
                if recipe.get_steps().is_empty() {
                    println!("Recipe must contain a non-zero number of steps");
                    return;
                }
                p = HeadChefService::new(recipe)
            }
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
        if p.check_failed() {
            println!(
                "Head chef has failed with no option for recovery: {}",
                p.get_failure_msg().unwrap()
            );
            return;
        }
    }

    println!("Chef-ing complete!");
}
