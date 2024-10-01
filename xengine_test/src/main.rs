use std::{arch::asm, fmt::format};

use xengine::{xanti_dbg, xfn, xfn_add, xfn_get, xfn_init, xstr, xtrash};

#[inline(never)]
fn xprint_text(text: &str) {
    xtrash!(2);
    xanti_dbg!({
        xtrash!(2);
        println!("{}", text);
        xtrash!(2);
    });
    xtrash!(2);
}

#[inline(never)]
fn xinput_text() -> String {
    xtrash!(2);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    xtrash!(2);
    input
}

#[inline(never)]
fn xcompare(a: &str, b: &str) -> bool {
    xtrash!(2);
    let mut value: bool = false;
    xanti_dbg!({
        xtrash!(2);
        if a == b {
            value = true;
        }
        xtrash!(2);
    });
    xtrash!(2);
    value
}

fn main() {
    let o_xprint_text: fn(text: &str) = xfn!(xprint_text);
    let o_xinput_text: fn() -> String = xfn!(xinput_text);
    let o_xcompare: fn(a: &str, b: &str) -> bool = xfn!(xcompare);

    o_xprint_text(xstr!("Enter password: ").as_str());
    let input = o_xinput_text();
    
    if o_xcompare(xstr!("ReadConsoleA").as_str(), input.as_str()) {
        o_xprint_text(xstr!("Success!").as_str());
    } else {
        o_xprint_text(xstr!("Failed!").as_str());
    }
}   