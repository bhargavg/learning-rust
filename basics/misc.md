- `format!()` is used to create a formatted `String`.
- Turbo fish is used to specify a generic function.
- Lifetime annotations don't change how long any of the references live. Rather, they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.
- All string literals have the `'static` lifetime.
- Update all installed rust binaries with: `cargo install $(cargo install --list | awk '/:$/ { print $1; }')`
- `eprintln!()` to write to `stderr`.
- `Iterator`s are lazy in general.
- `for` loop uses `into_iter`, which consumes the collection to produce it's iterator and yields the values.
- Along with `self`, `&self`, `&mut self`, we can also have `Box<Self>`, `Rc<Self>`, `Arc<Self>` & `Pin<&Self>`.

```Rust
// Examples of methods implemented on struct `Example`.
struct Example;
type Alias = Example;
trait Trait { type Output; }
impl Trait for Example { type Output = Example; }
impl Example {
    fn by_value(self: Self) {}
    fn by_ref(self: &Self) {}
    fn by_ref_mut(self: &mut Self) {}
    fn by_box(self: Box<Self>) {}
    fn by_rc(self: Rc<Self>) {}
    fn by_arc(self: Arc<Self>) {}
    fn by_pin(self: Pin<&Self>) {}
    fn explicit_type(self: Arc<Example>) {}
    fn with_lifetime<'a>(self: &'a Self) {}
    fn nested<'a>(self: &mut &'a Arc<Rc<Box<Alias>>>) {}
    fn via_projection(self: <Example as Trait>::Output) {}
}
```

# Lifetime elision rules:
- Each elided lifetime in input position becomes a distinct lifetime parameter.
- If there is exactly one input lifetime position (elided or not), that lifetime is assigned to all elided output lifetimes.
- If there are multiple input lifetime positions, but one of them is `&self` or `&mut self`, the lifetime of `self` is assigned to all elided output lifetimes.
- Otherwise, it is an error to elide an output lifetime.

# Testing:
- Test function can result a `Result` as well.
- There are 2 types of tests:
    - Unit tests, which are usually placed in the same file.
        - Need to annotate with `#[cfg(test)]` to not compile during normal builds.
    - Integration tests, which are placed in `tests` director, next to `src`.
        - Need not have `#[cfg(test)]` as they won't be compiled during normal builds.
- Use `-- --show-output` to see captured output from code during test.
- `cargo test PATTERN` will run all tests matching pattern.

## Testing attributes:
- `#[cfg(test)]` => Prevents test code from compiling during normal builds.
- `#[should_panic]` => The test function should panic, otherwise it will be recorded as a test failure.
- `#[ignore]` => Ignore the test function.
- `cargo test -- --ignored` => Run all ignored tests.
- `cargo test -- --include-ignored` => Run all tests, including ignored ones.


# Smart Pointers:
- `String` & `Vec<T>` are also smart pointers.
- A type can be made to behave like a smart pointer by using `Deref` trait.
- `std::mem::drop` => force drop.
- `Rc<RefCell<T>>` => Multiple owners that can mutate.
- `Mutex<T>` => Thread-safe version of `RefCell<T>`.
- `Rc<T>` => Increases strong ref count (`Rc::string_count`).
- `Weak<T>` => is created through `Rc::downgrade` method and it increases weak ref count `Rc::weak_count`.
- `Arc<Mutex<T>>` <= is same as => `RefCell<Rc<T>>`.
- `Rc<T>` => can create ref cycles
- `Mutex<T>` => can create deadlocks.
- Use cases for `Box<T>`, `Rc<T>` & `RefCell<T>`:
    - `Box<T>` 
        - Single owner
        - Immutable & mutable borrows
        - Compile time borrow checks
    - `Rc<T>`
        - Multiple owners
        - Immutable borrows
        - Compile time borrow checks
    - `RefCell<T>`
        - Single owner
        - Immutable & Mutable borrows
        - Runtime borrow checks
- `Rc::clone` is preferred than `A_RC_value.clone()`
- 

# Threads:
- `thread::spawn` to create an OS thread.
- `JoinHandle.join()` to join with the current thread.