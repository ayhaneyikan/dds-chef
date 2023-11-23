mod cook_chef;

use crate::cook_chef::CookChefService;

fn main() {
    // initialize cook chef service
    let mut cc = CookChefService::new();

    println!("Cook chef awaiting instructions");
    while !cc.check_completed() {
        cc.cycle();
        if cc.check_failed() {
            println!(
                "Head chef has failed with no option for recovery: {}",
                cc.get_failure_msg().unwrap()
            );
            return;
        }
    }

    println!("Cook chef completed cooking");
}
