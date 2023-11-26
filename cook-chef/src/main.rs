mod cook_chef;

use crate::cook_chef::CookChefService;

fn main() {
    // initialize cook chef service
    let mut cc = CookChefService::new();

    println!("Cook chef awaiting instructions");
    while !cc.check_completed() {
        cc.cycle();
        if let Some(error_msg) = cc.check_failed() {
            println!("Cook chef has failed: {}", error_msg);
            return;
        }
    }

    println!("Cook chef completed cooking");
}
