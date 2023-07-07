fn main() {
    option_as_ref();
}

fn option_as_ref() {
    let an_option = Option::Some("Hello".to_string());

    // To get the length of the string behind this `Option`, we can use a `map` function.
    // But, the following doesn't compile:
    //
    // ```
    // let str_length = an_option.map(|x| x.len());
    // println!("Length of {an_option:?} is: {str_length:?}");
    // ```
    //
    // This is because the option value is moved into `map` function.
    //
    // To avoid this, we can use `Option::as_ref` which
    // creates an intermediate `Option` value that gets consumed by the `map` function.
    //
    // Also, `as_ref` converts `&Option<T>` => `Option<&T>`
    let str_length = an_option.as_ref().map(|x| x.len());

    println!("Length of {an_option:?} is: {str_length:?}");
}
