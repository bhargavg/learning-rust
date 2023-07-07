fn main() {
    // When using a value expression in most place expression contexts,
    // a temporary unnamed memory location is created and initialized to that value.
    //
    // The expression evaluates to that location instead, except if promoted to a static.
    // The drop scope of teh temporary is usually the end of the enclosing statement.
    let a = &5;
    println!("{a}");
}
