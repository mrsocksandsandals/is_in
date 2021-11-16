# is_in
A small Rust library to provide a nicer way to check if something is in a slice.

### Usage
Add to your `Cargo.toml`:
```toml
[dependencies]
is_in = "0.1"
```

### Examples
```rust
use is_in::IsIn;
let slice1: [u8; 3] = [0, 3, 2];
println!("{}", 2_u8.is_in(&slice1)); // Prints "true".
```
