use std::sync::{Arc, Mutex};
const ITERATIONS: usize = 10000000;
const NUM_WORKING_THREADS: usize = 4;
fn main() {
    let tally = q3::score::ScoreTally::new(1);
    let tally = Arc::new(Mutex::new(tally));

    // Hardcoded Seed array. Produced by seed-array.py
    let a_counter = [0, 13, 10, 12, 5, 4, 16, 8, 2, 6, 9, 7, 14, 15, 3, 11];
    let b_counter = [0, 17, 15, 28, 21, 20, 11, 23, 7, 18, 24, 16, 6, 9, 5];
    crossbeam::scope(|scope| {
        let mut handles = vec![];
        for i in 0..NUM_WORKING_THREADS {
            // Seed the counters for each thread
            let mut a_counter = a_counter[i*ITERATIONS/NUM_WORKING_THREADS % a_counter.len()];
            let mut b_counter = b_counter[i*ITERATIONS/NUM_WORKING_THREADS % b_counter.len()];
            let shared_tally = tally.clone();
            let handle = scope.spawn(move |_inner_scope| {
                let mut tally = q3::score::ScoreTally::new(1);
                let start = i*ITERATIONS/NUM_WORKING_THREADS;
                let end = (i+1)*ITERATIONS/NUM_WORKING_THREADS;
                for _ in start..end {
                    tally.add(vec![("Alice", a_counter), ("Bob", b_counter)]);
                    // We can use the seed array to remove these operations but I assume that
                    // would be considered trivializing the problem, therefore i did not
                    a_counter = (a_counter * 5 + 13) % 17;
                    b_counter = (b_counter * 9 + 17) % 31;
                }
                shared_tally.lock().unwrap().add(tally.totals());

            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    })
    .unwrap();

    let tally = tally.lock().unwrap();
    let totals = tally.totals();
    for (candidate, score) in tally.totals().iter() {
        println!("{} got a score of {}", candidate, score);
    }

    assert_eq!(totals, vec![("Bob", 146666680), ("Alice", 84375000)]);
}
