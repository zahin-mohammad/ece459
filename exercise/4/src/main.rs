use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// You should modify main() to spawn threads and communicate using channels
fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    
    for received in rx {
        println!("Got: {}", received);
    }
}
