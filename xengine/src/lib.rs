use std::process::Output;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
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

    let mut rng = rand::thread_rng();


    // Generate the repeated print statements
    let mut repeated_code = quote! {};

    for _ in 0..repeat_count {
        let random_inst = rng.gen_range(0..=2);

        match random_inst {
            0 => {
                repeated_code.extend(quote! {
                    unsafe {
                        asm!(
                            "cmp rax, 0x1488",
                            "jne 2f",
                            "jmp rax",
                            "2:"
                        );
                    };
                });
            }
            1 => {
                repeated_code.extend(quote! {
                    let code: Vec<u8> = vec![
                        0xFF, 0xEE, 0xAA, 0xBB, 0x01, 0x02, 0x03, 0x04,
                    ];
                
                    let code_ptr = code.as_ptr();
                
                    unsafe {
                        asm!(
                            "mov rax, {}",
                            in(reg) code_ptr,
                        );
                        asm!(
                            "cmp rax, 0x13EF",
                            "jne 2f",
                            "jmp rax",
                            "2:",
                        );
                    }
                
                });
            },
            2 => {
                repeated_code.extend(quote! {
                    unsafe {
                    asm!{
                        "2:",
                        "cmp rax, 0x78F",
                        "je 2b"
                    };
                    };
                
                });
            },
            _ => {
                panic!("Invalid instruction");
            }
        }
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
                let code: Vec<u8> = vec![
                    0xFF, 0xEE, 0xAA, 0xBB, 0x01, 0x02, 0x03, 0x04,
                ];
            
                let code_ptr = code.as_ptr();
            
                unsafe {
                    asm!(
                        "mov rax, {}",
                        "jmp rax",
                        in(reg) code_ptr,
                    );
                }
            }
            xtrash!(4);
        }
    };

    TokenStream::from(output)
}

#[proc_macro]
pub fn xfn_init(input: TokenStream) -> TokenStream {

    let output = quote!{
        
            let mut xfn_list: Vec<usize> = Vec::new();
        
    };

    TokenStream::from(output)
}

#[proc_macro]
pub fn xfn_add(input: TokenStream) -> TokenStream {
    //parse name: String and fn: usize
    let input_data = parse_macro_input!(input as Expr);

    let output = quote!{
        
            xfn_list.push(#input_data as usize);
        
    };

    TokenStream::from(output)
}

#[proc_macro]
pub fn xfn_get(input: TokenStream) -> TokenStream {
    //parse name: String and fn: usize
    let input_data = parse_macro_input!(input as LitInt);
    
    let output = quote!{
        
            unsafe { std::mem::transmute(xfn_list[#input_data]) }
        
    };

    TokenStream::from(output)

}

#[proc_macro]
pub fn xfn(input: TokenStream) -> TokenStream {
    //parse name: String and fn: usize
    let input_data = parse_macro_input!(input as Expr);
    
    let output = quote!{
        {
            xtrash!(1);
            xfn_init!();
            xtrash!(1);
            xfn_add!(#input_data as usize);
            xtrash!(1);
            xfn_get!(0)
        }
        
    };

    TokenStream::from(output)
}