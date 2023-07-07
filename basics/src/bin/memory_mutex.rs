use std::{
    sync::{Arc, Mutex},
    thread,
};
fn main() {
    understanding_mutex_usage();
    mutex_with_threads();
}

fn understanding_mutex_usage() {
    let m = Mutex::new(5);
    // To modify the value guarded by `Mutex`, we have to acquire a lock
    // `Mutex::lock` returns a `MutexGuard` which is a smart pointer for
    // accessing / updating the guarded value.

    let mut num = m.lock().unwrap();

    // We now hafe locked the `Mutex`.
    // Printing the `Mutex` will show as:
    //   Mutex { data: <locked>, poisoned: false, .. }
    println!("{m:?}");

    // We can update the value held by mutex through this guard (smart pointer).
    *num = 20;

    // When this `MutexGuard` is deallocated, the `Mutex` is unlocked.
    drop(num);

    // As the mutex is now locked again, we can read the value.
    // This now prints:
    //   Mutex { data: 20, poisoned: false, .. }
    println!("{m:?}");

    // If we just want to update, we can do:
    *m.lock().unwrap() = 30;
    println!("{m:?}");
}

fn mutex_with_threads() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            *counter.lock().unwrap() += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
        println!("Result: {}", *counter.lock().unwrap());
    }
}
