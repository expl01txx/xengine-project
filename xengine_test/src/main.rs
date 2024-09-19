use std::arch::asm;

use xengine::{xanti_dbg, xstr, xtrash};

fn main() {
    xtrash!(32);
    xanti_dbg!({
        println!("{}", xstr!("Enter password: "));
    });
    xtrash!(4);
    let mut password = String::new();
    std::io::stdin().read_line(&mut password).unwrap();
    xtrash!(4);
    xanti_dbg!({
        xtrash!(4);
        if password == xstr!("rust") {
            xtrash!(4);
            println!("{}", xstr!("Access granted"));
        } else {
            xtrash!(4);
            println!("{}", xstr!("Access denied"));
        }
        xtrash!(4);
    });
    xtrash!(16);
    let mut tmp = String::new();
    std::io::stdin().read_line(&mut tmp).unwrap();
    xtrash!(16);
}