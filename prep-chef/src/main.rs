mod prep_chef;

use crate::prep_chef::PrepChefService;

fn main() {
    // initialize prep chef service
    let mut pc = PrepChefService::new();

    println!("Prep chef beginning preparations");
    while !pc.check_completed() {
        pc.cycle();
    }

    println!("Prep chef completed preparations");
}
