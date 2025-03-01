use std::{arch::asm, fmt::format, io, process::exit};

use xengine::{xanti_dbg, xfn,  xstr, xtrash};




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