use std::thread;

fn main() {
    closure_immutable_borrows();
    closure_mutable_borrow();
    closure_ownership_with_move_keyword();
}

fn closure_immutable_borrows() {
    let list = vec![1, 2, 3, 4];

    // Closure just takes an immutable ref as that is what's least required.
    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}

fn closure_mutable_borrow() {
    let mut list = vec![1, 2, 3, 4];

    // The closure has to be marked as `mut` because:
    //   - The closure's type will be `impl FnMut()`, as it is mutating the captured list.
    //   - The FnMut's closure variable has to be marked as `mut` because it modifies it's env.
    //
    // The `list` is borrowed mutably by the closure.
    let mut takes_mut_ref = || list.push(5);
    takes_mut_ref();
    println!("After calling closure: {list:?}");

    // Uncommenting the following line will cause compiler error.
    // This is because, as long as `takes_mut_ref` closure isn't deallocated,
    // it holds a mutable ref to `list`. As per Rust's ownership rules,
    // there can't be any immutable refs (used in `println` above) when there is
    // a mutable ref (used in `takes_mut_ref` closure.)
    // takes_mut_ref();
}

fn closure_ownership_with_move_keyword() {
    let list = vec![1, 2, 3, 4];

    // `move` the captured varaibles into the closure.
    thread::spawn(move || {
        println!("From thread: {list:?}");
    })
    .join()
    .unwrap();

    // The following line will cause compiler error.
    // This is because `list` is no longer available in this scope.
    // (`move`d into the closure.)
    // ```
    // println!("From outside: {:?}", list);
    // ```
}
