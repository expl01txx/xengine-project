use proc_macro::TokenStream;
use quote::quote;
use rand::Rng;
use syn::{parse_macro_input, Expr,LitInt, LitStr};

#[proc_macro]
pub fn xstr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let plain_text = input.value();

    // Simple XOR encryption with a fixed key
    let mut rng = rand::thread_rng();
    let key = rng.gen_range(0x10..=0xEE);
    let encrypted: String = plain_text.chars().map(|c| (c as u8 ^ key) as char).collect();

    // Generate the decryption code
    let expanded = quote! {
        {
            const ENCRYPTED: &'static str = #encrypted;
            let decrypted: String = ENCRYPTED.chars().map(|c| (c as u8 ^ #key) as char).collect();
            decrypted
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn xtrash(input: TokenStream) -> TokenStream {
    // Parse the input as an integer literal
    let repeat_count = parse_macro_input!(input as LitInt);

    // Get the number of times to repeat
    let repeat_count: usize = repeat_count.base10_parse().expect("Failed to parse integer");


    // Generate the repeated print statements
    let mut repeated_code = quote! {};

    for _ in 0..repeat_count {
        repeated_code.extend(quote! {
            unsafe {
                asm!(
                    "2:",
                    "cmp rax, 0xEE",
                    "je 2b",
                    "cmp rbx, 0xAA",
                    "je 2b",
                    "cmp rax, 0",
                    "je 3f",
                    "cmp rax, 0xAA",
                    "4: je 4b",
                    "3:"
                );
            };
        });
    }

    TokenStream::from(repeated_code)
}

#[proc_macro]
pub fn xanti_dbg(input: TokenStream) -> TokenStream {
    // Parse the input into a Rust block of code
    let input_block = parse_macro_input!(input as Expr);

    // Inject timing code using quote
    let output = quote! {
        {   
            xtrash!(4);
            let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
            xtrash!(4);

            let result = (|| #input_block)();
            xtrash!(4);

            let new_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
            xtrash!(4);

            if new_time - current_time > 10 {
                std::process::exit(0);
            }
            xtrash!(4);
        }
    };

    TokenStream::from(output)
}