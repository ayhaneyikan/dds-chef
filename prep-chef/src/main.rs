mod prep_chef;

use crate::prep_chef::PrepChefService;

fn main() {
    // initialize prep chef service
    let mut pc = PrepChefService::new();

    println!("Prep chef beginning preparations");
    while !pc.check_completed() {
        pc.cycle();
        if pc.check_failed() {
            println!(
                "Head chef has failed with no option for recovery: {}",
                pc.get_failure_msg().unwrap()
            );
            return;
        }
    }

    println!("Prep chef completed preparations");
}
