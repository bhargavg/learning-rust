fn main() {
    doesnot_live_long_enough_error();
}

// - ellision doesn't work here as we have multiple refs as arguments
//   and compiler can't figure out which lifecycle to pick. Hence explicit
//   lifetimes are required.
// - Parameters should be annotated with same lifetime param because we can't
//   refs of mulitiple lifetimes from a function.
fn longest_among<'a>(left: &'a str, right: &'a str) -> &'a str {
    if right.len() > left.len() {
        right
    } else {
        left
    }
}

fn doesnot_live_long_enough_error() {
    let l = String::from("Hello");

    #[allow(clippy::needless_late_init)]
    let longest: &str;

    // Because `r` has smaller lifetime (valid only inside block),
    // compiler doesn't allow it.
    //
    // Uncomment following braces for compiler error:
    // {
    let r = String::from(" World");
    longest = longest_among(&l, &r);
    // }

    println!("Longest is: {longest}");
}
