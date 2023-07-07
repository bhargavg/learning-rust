# Crate
- Smallest amount of code that the rust compiler considers at a time.

## Binary Crate
- `src/main.rs` => makes it a binary crate
- A package can have multiple binary crates by:
    - placing files in the `src/bin` directory: each file will be a separate binary crate.
    - creating a different folders, each with their own `main.rs`.

## Crate Root
- `src/lib.rs` for library crate.
- `src/main.rs` for binary crate.

# Package
- Collection of crates.
- Contains `Cargo.toml`.
- Can contain multiple binary crates, but only one library crate.

# Modules
## Declaring modules
- Compiler should be made aware of a module, by placing `mod MODULE_NAME;` in [crate root][#crate-root]. In other words, a module must be added to the crate tree for it to be picked up by the compiler.
- Module's code should be in:
    - Inline: `mod MODULE_NAME { MODULE CODE }`
    - In the file: `src/MODULE_NAME.rs`
    - In the file: `src/MODULE_NAME/mod.rs`
- Submodules can be defined in similar way.
- Modules path:
    - `crate::MODULE::SUBMODULE::SUBMODULE`
    - `super::MODULE::SUBMODULE::SUBMODULE`
    - `self::MODULE::SUBMODULE::SUBMODULE`
- Module's internals are private by default.