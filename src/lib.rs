use std::{collections::HashMap, fs, process::Output, sync::OnceLock};

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use rand::{Rng, distributions::Alphanumeric, seq::{IteratorRandom, SliceRandom}};
use syn::{parse_macro_input, parse_quote, AttributeArgs, Expr, ItemFn, LitInt, LitStr, Stmt};

static LABELS_VEC: OnceLock<std::sync::Mutex<Vec<String>>> = OnceLock::new();

fn get_labels_vec() -> &'static std::sync::Mutex<Vec<String>> {
    LABELS_VEC.get_or_init(|| std::sync::Mutex::new(Vec::new()))
}

#[proc_macro]
pub fn xstr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let plain_text = input.value();

    let mut rng = rand::thread_rng();
    let key = rng.gen_range(0x10..=0xEE);
    let encrypted: String = plain_text.chars().map(|c| (c as u8 ^ key) as char).collect();

    let expanded = quote! {
        {
            const ENCRYPTED: &'static str = #encrypted;
            let decrypted: String = ENCRYPTED.chars().map(|c| (c as u8 ^ #key) as char).collect();
            decrypted
        }
    };

    TokenStream::from(expanded)
}

fn generate_random_string(len: usize) -> String {
    let s: String = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(len)
    .map(char::from)
    .collect();
    "LABEL_".to_owned() + &s
}

fn get_random_label() -> Option<String> {
    let labels_vec = get_labels_vec().lock().unwrap();
    
    if labels_vec.is_empty() {
        return None;
    }
    
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..labels_vec.len());
    
    Some(labels_vec[random_index].clone())
}

#[proc_macro]
pub fn xtrash(input: TokenStream) -> TokenStream {
    let repeat_count = parse_macro_input!(input as LitInt);

    let repeat_count: usize = repeat_count.base10_parse().expect("Failed to parse integer");

    let mut rng = rand::thread_rng();

    let mut repeated_code = quote! {};

    for _ in 0..repeat_count {

        let rnd_label = get_random_label();
        let mut labels_vec = get_labels_vec().lock().unwrap();
        if rnd_label.is_none() {
            let label_name = generate_random_string(8) + &labels_vec.len().to_string();
            repeated_code.extend(quote! {
                unsafe {
                    asm!(
                        "cmp rax, 0x7432",
                        "jne 2f",
                        concat!(".global ", #label_name, "\n", #label_name, ":"),
                        "push rax",
                        "sub rsp, 0x11",
                        "add rax, 0x100",
                        "call rax",
                        "add rsp, 0x09",
                        "pop rax",
                        "ret",
                        "2:"

                    );
                };
            });
            labels_vec.push(label_name);
            continue;
        }

        let random_inst = rng.gen_range(0..=4);
        match random_inst {
            0 => {
                let label_name = generate_random_string(8) + &labels_vec.len().to_string();
                repeated_code.extend(quote! {
                    unsafe {
                        asm!(
                            "cmp rax, 0x1488ffcc",
                            "jne 2f",
                            concat!(stringify!(#label_name), ":"),
                            "push rax",
                            "sub rsp, 0x8",
                            "add rax, 0x54",
                            "call rax",
                            "add rsp, 0x4",
                            "pop rax",
                            "ret",
                            "jmp rax",
                            "2:"

                        );
                    };
                });
                labels_vec.push(label_name);
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
                            "2:"
                        );
                    }
                
                });
            },
            2 => {
                let cmd = "je ".to_string() + &rnd_label.unwrap();
                repeated_code.extend(quote! {
                    unsafe {
                    asm!{
                        "2:",
                        "cmp rax, 0x78F",
                        "je 2b",
                        "cmp rax, 0x1F0402F",
                        #cmd
                    };
                    };
                });
            },
            3 => {
                let cmd = "jmp ".to_string() + &rnd_label.unwrap();
                let label_name = generate_random_string(8) + &labels_vec.len().to_string();
                repeated_code.extend(quote! {
    
                    let code: Vec<u8> = vec![
                        0xFF, 0xEE, 0xAA, 0xBB, 0x01, 0x02, 0x03, 0x04,
                    ];
                
                    let code_ptr = code.as_ptr();
                    unsafe {
                        asm!{
                            "cmp r10, 0xaaff",
                            "jne 2f",
                            concat!(".global ", #label_name, "\n", #label_name, ":"),
                            #cmd,
                            "int3",
                            "push {0}",
                            "ret",
                            "2:",
                            in(reg) code_ptr,
                        };
                    };
                
                });
                labels_vec.push(label_name);
            },
            4 => {
                let cmd = "jmp ".to_string() + &rnd_label.unwrap();
                repeated_code.extend(quote! {

                    unsafe {
                        asm!{
                            "cmp r10, 0xaaff",
                            "jne 2f",
                            #cmd,
                            "2:",
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
    let input_block = parse_macro_input!(input as Expr);

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
pub fn xinclude_bytes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let path = input.value();
    
    let bytes = match fs::read(format!("src/{}", &path)) {
        Ok(bytes) => bytes,
        Err(e) => {
            return syn::Error::new(
                input.span(),
                format!("Failed to read file '{}': {}", path, e)
            )
            .to_compile_error()
            .into();
        }
    };
    
    let mut rng = rand::thread_rng();
    let key = rng.gen_range(0x10..=0xEE);
    let bytes: Vec<u8> = bytes.iter().map(|c| (*c as u8 ^ key) as u8).collect();
    let bytes = bytes.as_slice();
    let bytes_lit = syn::LitByteStr::new(&bytes, input.span());
    
    let expanded = quote! {
        {
            let encrypted = #bytes_lit;
            let bytes: Vec<u8> = encrypted.iter().map(|c| (*c as u8 ^ #key) as u8).collect();
            bytes
        }
    };
    
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn xfn(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(attr as AttributeArgs);
    let trash_amount: usize = match attr_args.as_slice() {
        [syn::NestedMeta::Lit(syn::Lit::Int(lit))] => lit.base10_parse().unwrap(),
        _ => panic!("Expected a single integer argument, e.g., #[xfn(4)]"),
    };

    let input_fn = parse_macro_input!(item as ItemFn);
    let vis = input_fn.vis;
    let sig = input_fn.sig;

    let mut block = input_fn.block;

    let modified_stmts: Vec<syn::Stmt> = block
        .stmts
        .into_iter()
        .flat_map(|stmt| {
            let xtrash_stmt = {
                let amount = trash_amount;
                quote! {
                    {
                        use std::arch::asm;
                        xtrash!(#amount);
                    }
                }
            };
            vec![syn::parse2(xtrash_stmt).unwrap(), stmt]
        })
        .collect();

    block.stmts = modified_stmts;

    let output = quote! {
        #vis #sig #block
    };

    output.into()
}