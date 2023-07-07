// A `Box` is used in following cases:
//
// - For creating recursive data structure (indirection).
// - For heap alocating something that has a single owner (`Rc` for multiple owners).
fn main() {
    let b = Box::new(5);
    println!("{b}");
}
