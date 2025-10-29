# Simple rustlang obfuscation library based on proc-macro and inline assembly for x86_64

### Example:
```rust
#[xfn(64)]
fn main() {
    println!("{}", xstr!("Enter the password:"));

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Remove any trailing newline characters
    let input = input.trim();

    let correct_password = &xstr!("rust");

    if input == correct_password {
        println!("{}", xstr!("Correct!"));
    } else {
        println!("{}", xstr!("Incorrect!"));
    }
    exit(0);
}   
```
