use std::{thread, time::Duration};

fn main() {
    using_join();
    using_move();
}

fn using_join() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // `unwrap` is called here to ensure that we don't proceed if there is an error.
    handle.join().unwrap();

    println!("This will be printed after everything");
}

fn using_move() {
    let v = vec![1, 2, 3, 4, 5];

    // The closure passed to `thread::spawn` uses `v` from surrounding environment.
    // rust cannot guarantee that `v` will live long enough to be able to use it from closure.
    // Hence, we have to move it to closure, ie., transfer ownership to the closure.

    let handle = thread::spawn(move || {
        println!("here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
