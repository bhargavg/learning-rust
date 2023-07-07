fn main() {
    multiple_immutable_refs_can_exist();
    only_one_mut_ref_can_exist();
}

fn multiple_immutable_refs_can_exist() {
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{r1} {r2}");
}

#[allow(unused_variables)]
fn only_one_mut_ref_can_exist() {
    let mut s = String::from("hello");

    // If a mutable ref exists, there should be no other ref
    // ie., `&mut` cannot co-exist with `&` and `&mut`
    //
    // Reason:
    // Users of an immutable ref don't expect the value to suddenly change.
    // However, multiple immutable refs are allowed as no one has the ability
    // to change it.

    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;

    // This fails to compile
    // ```
    // println!("{r1} {r2} {r3}");
    // ```

    // This works because `r2` & `r3` will be dropped as they are no longer being referenced.
    //
    // This is called Non-Lexical Lifetimes (NLL for short)
    println!("{r3}");
}
