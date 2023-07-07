use std::{sync::mpsc, thread, vec};

fn main() {
    using_channels();
}

fn using_channels() {
    // mpsc => multi producer single consumer
    //
    // `Sender<T>` & `Receiver<T>` are marked as `!Send`
    // hence they cannot be safely shared between threads.
    let (tx, rx) = mpsc::channel::<Message>();

    for i in 0..10 {
        // For each thread, clone a separate `Sender<T>`
        // as it will be exclusive (read: `move`d) into the thread's closure.
        let tx = tx.clone();
        thread::spawn(move || {
            let contents = vec!["hi", "from", "the", "thread"];

            for c in contents {
                // The data that we `send` through `Sender<T>` should
                // own all it's contents. Hence `c.to_owned()`.
                let message = Message {
                    thread_id: i,
                    contents: c.to_owned(),
                };

                // `send` consumes/takes-ownership of the argument
                tx.send(message).unwrap();
            }

            println!("Done sending values from thread#{i}");
        });
    }

    // TODO: Figure out how to properly close `Sender<T>` so that we can use `for` loop.
    //
    // `Receiver<T>` can vend an iterator for `for` loop to consume like:
    // ```
    // for msg in rx {}
    // ```
    // But this will be blocking if there are any `Sender<T>`s left.
    //
    // To avoid this, as we already know how many messages all threads will send,
    // we can just ask just for those many times.
    for _ in 0..(4 * 10) {
        let m = rx.recv().unwrap();
        println!("Received: {} from thread#:{}", m.contents, m.thread_id);
    }
}

#[derive(Debug)]
struct Message {
    thread_id: i32,
    contents: String,
}
