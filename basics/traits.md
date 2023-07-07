# Default implementations

```Rust
triat Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
}
```

# Orphan rule
- We can only implement a trait on a type only if at-least one of the trait or the type is local to our crate.

# Trait parameters
```Rust
// We can pass two different types to both params as long as they both implement TRAIT
fn method(param1: &impl TRAIT, &impl TRAIT)
```

This is exactly same as:
```Rust
// Both params1 & param2 have to be of same type that implements TRAIT 
fn method<T: TRAIT>(param1: &T, param2: &T)
```

This is exactly same as:
```Rust
// Used for redability, by pusing type params to right side of where clause
fn method<T>(param1: &T, param2: &T) where T: impl TRAIT
```

## Multiple Trait bounds
```Rust
fn method(param1: &(impl TRAIT1 + TRAIT2));
fn method<T: (TRAIT1 + TRAIT2 ...)(param1: &T);
```

# Trait bounds in return values
- Supports all variations as trait parameters
- All possible return values should be of same type.

# Conditionally implement methods
- Example: `to_string` method.

```Rust
impl<T: TRAIT1 + TRAIT2> TYPE<T> {
    ...
}
```