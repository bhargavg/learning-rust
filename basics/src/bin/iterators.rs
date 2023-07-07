fn main() {
    // By default, `for` loop uses `into_iter`
    //
    //      `iter` => This borrows each element of the collection through each iteration.
    //  `iter_mut` => This mutably borrows each element of the collection, allowing for the collection to be modified in place.
    // `into_iter` => This consumes the collection so that on each iteration the exact data is provided.
    itar_takes_immutable_ref_and_vends_immutable_ref_values();
    consuming_adaptors();
    iterator_adaptors();
}

fn itar_takes_immutable_ref_and_vends_immutable_ref_values() {
    let v1 = vec![1, 2, 3, 4, 5];

    // `iter()` method returns an `impl Iterator<Item = &i32>`
    // This can be verified as follows:
    // ```
    // let iterator: Box<dyn Iterator<Item = &i32>> = Box::new(v1.iter());
    // ```

    let v2 = v1
        .iter()
        // This should've been `*v + 1`, but `v + 1` also works as `Add` is defined for refs too
        .map(|v: &i32| v + 1)
        // Same reason, this should've been `*v % 2 == 0`
        .filter(|v: &i32| v % 2 == 0)
        // Doesn't work here as equality isn't defined for refs
        .filter(|v: &i32| *v == 2)
        .collect::<Vec<_>>();

    println!("{v1:?} {v2:?}");
}

fn consuming_adaptors() {
    let v1 = vec![1, 2, 3, 4, 5];
    let iter = v1.iter();

    // `sum` method takes ownership (read: consumes) `self.
    println!("Sum = {}", iter.sum::<i32>());
}

fn iterator_adaptors() {
    // Iterator adaptors are methods on `Iterator` that produce
    // different iterators instead of consuming them.

    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = v1.iter().map(|x| x + 1).collect::<Vec<_>>();

    println!("{v1:?} {v2:?}");
}
