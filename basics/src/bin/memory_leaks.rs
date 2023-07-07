use std::{cell::RefCell, rc::Rc};

fn main() {
    // `a` => Immutable `List::Cons` with a mutable next link
    // `a` => Cons(5, Nil)
    let a = Rc::new(List::Cons(5, RefCell::new(Rc::new(List::Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    // `b` => Same as `a` above, with `a` added to `b`s end
    // `b` => Cons(10, a)
    //     => Cons(10, Cons(5, Nil))
    let b = Rc::new(List::Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next = {:?}", b.tail());

    // We are now adding `b` to `a`'s  end, causing memory leak
    // `a` => Cons(5, b)
    //     => Cons(5, Cons(10, a))
    //                         ^--Loop
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle.
    // it will overflow the stack
    // ```
    // println!("a next item = {:?}", a.tail());
    // ```
}

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            List::Cons(_, next) => Some(next),
            List::Nil => None,
        }
    }
}
