use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new()),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    dbg!(leaf.parent.borrow().upgrade());
}

#[derive(Debug)]
#[allow(dead_code)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}
