# Concrete constraints

```Rust
struct Point<T> {
    x: T,
    y: T,
}

impl Point<i32> {
    fn distance_to_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

# Monomorphization
- Converts generic code into specific code by filling in the concrete types that are used when compiled.

# Turbo fish
```
method::<...>
```