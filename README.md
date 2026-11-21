# xengine

Rust procedural macros for compile-time literal and file obfuscation and x86-64
control-flow instrumentation. The flow macros support Linux and Windows x86-64
with Cargo's default codegen settings.

## Literals and files

| Macro | Returns | Input |
| --- | --- | --- |
| `xstr!("café")` | `String` | A UTF-8 string literal |
| `xbytes!(b"\x00\xFF")` | `Vec<u8>` | A byte string literal |
| `xcstr!("ready")` | `CString` | A UTF-8 string literal with no interior NUL bytes |
| `xinclude_str!("assets/banner.txt")` | `String` | A UTF-8 text file |
| `xinclude_bytes!("assets/logo.bin")` | `Vec<u8>` | A file containing arbitrary bytes |

The macros encode values at compile time and decode them into owned values at
runtime. The encoding is reversible and the key is embedded in the binary.
Keep secrets outside the binary.

File paths are relative to the consuming crate's `src` directory and use the
build host's path conventions. For example, `xinclude_str!("assets/banner.txt")`
reads `src/assets/banner.txt`. Cargo rebuilds when either file macro's input
changes. Invalid UTF-8 in a text file and interior NUL bytes in a C string
literal cause compile-time errors.

## Control flow and timing

- `xtrash!(count)` inserts the requested number of local assembly branches.
  `#[xfn(count)]` inserts them before each top-level statement in a function.
  A count of zero emits no instrumentation.
- `xanti_dbg!(expression)` evaluates the expression once and returns its value.
  It panics if the expression takes longer than 10 ms.

```rust
use xengine::{xbytes, xcstr, xfn, xstr};

#[xfn(4)]
fn main() {
    println!("{}", xstr!("Hello, café 🦀!"));
    assert_eq!(xbytes!(b"\x00\xFF"), [0, 255]);
    assert_eq!(xcstr!("ready").as_bytes_with_nul(), b"ready\0");
}
```

## Tests

Run `cargo test --locked` and `cargo test --locked --release`. The suite checks
macro behavior and scans the test executable for the original plaintext
markers.
