use std::thread;

static N: i32 = 10;  // number of threads to spawn

fn thread_fn(thread: i32) {
    println!("Hello from thread {}!", thread);
}

fn main() {
    // Dynamic array (go slice)
    let mut children = vec![];
    for x in 0..N {
        // Closure passes in value to fn
        // TODO: what does move do?
        children.push(thread::spawn(move || {
            thread_fn(x)
        }))
    }
    // TODO: When to use children.iter() ?
    for child in children {
        let _ = child.join();
    }
}
