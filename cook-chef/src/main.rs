mod cook_chef;

use crate::cook_chef::CookChefService;

fn main() {
    // initialize cook chef service
    let mut cc = CookChefService::new();

    println!("Cook chef begining cooking");
    while !cc.check_completed() {
        cc.cycle();
    }

    println!("Cook chef completed cooking");
}
