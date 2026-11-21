# xengine

Rust procedural macros for compile-time string and file obfuscation and x86-64
control-flow instrumentation. The flow macros support x86-64 Linux and Windows
with Cargo's default codegen settings.

- `xstr!("literal")` returns an owned UTF-8 `String`.
- `xinclude_bytes!("asset.bin")` returns an owned `Vec<u8>`. Relative paths are
  resolved from the consuming crate's `src` directory. Path separators follow
  the build host's filesystem conventions.
- `xtrash!(count)` inserts the requested number of local assembly branches.
  `#[xfn(count)]` does the same before each top-level statement in a function.
  A count of zero leaves the code unchanged.
- `xanti_dbg!(expression)` evaluates the expression once and returns its value.
  It panics if the expression takes longer than 10 ms.

The string and file macros use reversible encoding with an embedded key. Use
them for obfuscation of literals and assets. Keep secrets outside the binary.

```rust
use xengine::{xfn, xstr};

#[xfn(4)]
fn main() {
    println!("{}", xstr!("Hello, café 🦀!"));
}
```

Run the test suite with `cargo test --locked` and
`cargo test --locked --release`.
