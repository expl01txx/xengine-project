# Rustlang obfuscation library based on proc-macro and inline assembly for x86_64

### Exampl usage:
```rust
use xengine::*;

#[xfn(64)]
fn main() {
    println!("{}", xstr!("Enter the password:"));

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    let input = input.trim();

    let correct_password = &xstr!("rust");

    if input == correct_password {
        println!("{}", xstr!("Correct!"));
    } else {
        println!("{}", xstr!("Incorrect!"));
    }
    std::process::exit(0);
}  
```
