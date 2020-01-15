![Crates.io](https://img.shields.io/crates/l/source_loc)
![Crates.io](https://img.shields.io/crates/v/source_loc)

# source_loc

Utility for capturing source file location at compile time.
Much less overhead than `Backtrace`, but, of course you don't get a full
backtrace.

## Example

```rust
use source_loc::source_loc;

let loc = source_loc!();
assert_eq!(
    "src/lib.rs:6:11",
    &loc.to_string(),
);
```
