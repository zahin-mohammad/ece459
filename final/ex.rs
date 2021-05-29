
let some_var = AtomicUsize::new(5);
{
    let some_var = some_var.clone();
    let t = thread::spawn(move || some_var.compare_and_swap(5, 10, Ordering::Relaxed));
}

some_var.compare_and_swap(10, 12, Ordering::Relaxed)

// If t thread executes first, then some_var holds 12
// If main thread executes first, then some_var holds 10