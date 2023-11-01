mod some_subscriber;

use some_subscriber::SubscriberBase;

fn main() {
    // initialize subscriber service
    let mut s = SubscriberBase::new();

    println!("Running subscriber");
    while !s.check_completed() {
        s.cycle();
    }

    println!("Subscriber completed running successfully!");
}
