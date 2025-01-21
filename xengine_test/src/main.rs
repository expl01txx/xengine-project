use std::{arch::asm, fmt::format};

use xengine::{xanti_dbg, xfn, xfn_add, xfn_get, xfn_init, xstr, xtrash};

#[inline(never)]
fn xprint_text(text: &str) {
    xtrash!(16);
    xanti_dbg!({
        xtrash!(2);
        println!("{}", text);
        xtrash!(2);
    });
    xtrash!(16);
}

#[inline(never)]
fn xinput_text() -> String {
    xtrash!(16);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    xtrash!(16);
    input
}

#[inline(never)]
fn xcompare(a: &str, b: &str) -> bool {
    xtrash!(16);
    let mut value: bool = false;
    xanti_dbg!({
        xtrash!(16);
        if a == b {
            value = true;
        }
        xtrash!(16);
    });
    xtrash!(16);
    value
}

fn main() {
    xtrash!(16);
    let o_xprint_text: fn(text: &str) = xfn!(xprint_text);
    xtrash!(16);
    let o_xinput_text: fn() -> String = xfn!(xinput_text);
    xtrash!(16);
    let o_xcompare: fn(a: &str, b: &str) -> bool = xfn!(xcompare);
    xtrash!(16);
    o_xprint_text(xstr!("Enter password: ").as_str());
    xtrash!(16);
    let input = o_xinput_text();
    xtrash!(16);
    if o_xcompare(xstr!("ReadConsoleA").as_str(), input.as_str()) {
        xtrash!(16);
        o_xprint_text(xstr!("Success!").as_str());
    } else {
        xtrash!(16);
        o_xprint_text(xstr!("Failed!").as_str());
    }
}   