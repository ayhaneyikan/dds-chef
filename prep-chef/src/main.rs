mod prep_chef;

use crate::prep_chef::PrepChefService;

fn main() {
    // initialize prep chef service
    let mut pc = PrepChefService::new();

    println!("Prep chef awaiting instructions");
    while !pc.check_completed() {
        pc.cycle();
        if let Some(error_msg) = pc.check_failed() {
            println!("Prep chef has failed: {}", error_msg);
            return;
        }
    }

    println!("Prep chef completed preparations");
}
