fn main() {
    // `String` is a specialized pointer to an `str` slice,
    // and `Vec<T>` is a specialized pointer to a `[T]` slice.
    //
    // A `String` slice, aka `str`, is just a `[u8]`.
    //
    // `[u8]` => - contigous segment of `u8`s.
    //           - unsized. Each element is `u8` (size known),
    //                      but, there is no information about how many are there.
    //
    // Because `[u8]` is unsized, Rust doesn't allow us to do the following:
    //
    // ```
    // let foo: str = "abc"[..];
    // ```
    //
    // Hence, slices are always references.
    // See more: https://stackoverflow.com/a/61151916

    invalid_slice();
}

#[allow(unused_variables)]
fn invalid_slice() {
    let mut s = String::from("hello, world!");
    let word = first_word(&s);

    // This compiles.
    //
    // Even though we are clearing the contents of source string while we still have slice,
    // Rust actually drops `word` as soon as it's initialized with a slice,
    // because it isn't getting used anywhere.
    s.clear();

    // This doesn't compile.
    //
    // We are trying to access a slice (`word`) after clearing out the source String (`s`).
    //
    // Rust doesn't allow us to do this because of the following ownership rule:
    //    We can't have a mutable ref, while a normal ref is still active.
    //
    // Normal ref is created when we call `first_word` and that ref stays till `word` is not-dropped.
    // Mutable ref is created when we call `s.clear()`, because `clear()` needs to modify the contents.
    //
    // ```
    // s.clear();
    // println!("{word}");
    // ```
}

fn first_word(s: &str) -> &str {
    for (idx, &byte) in s.as_bytes().iter().enumerate() {
        if byte == b' ' {
            return &s[..idx];
        }
    }

    s // short for: &s[..]
}
