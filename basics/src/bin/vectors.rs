fn main() {
    mutation_inside_for_loop();
}

fn mutation_inside_for_loop() {
    let mut v = vec![1, 2, 3, 4];

    for i in &mut v {
        *i *= 2;
    }

    println!("{v:?}");
}
