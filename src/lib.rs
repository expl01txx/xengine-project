//! Compile-time string and byte obfuscation and x86-64 control-flow instrumentation.

use std::{env, fs, path::PathBuf};

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use rand::Rng;
use syn::{parse_macro_input, parse_quote, Expr, ItemFn, LitByteStr, LitInt, LitStr};

/// Obfuscates a UTF-8 string literal at compile time and returns an owned `String`.
///
/// ```
/// use xengine::xstr;
///
/// assert_eq!(xstr!("café 🦀"), "café 🦀");
/// ```
#[proc_macro]
pub fn xstr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    obfuscated_string(input.value().as_bytes(), input.span()).into()
}

fn obfuscated_string(bytes: &[u8], span: proc_macro2::Span) -> TokenStream2 {
    let bytes = obfuscated_bytes(bytes, span);
    quote! {
        ::std::string::String::from_utf8(#bytes)
            .expect("xengine encoded text must decode to valid UTF-8")
    }
}

/// Obfuscates a byte string literal at compile time and returns an owned `Vec<u8>`.
///
/// ```
/// use xengine::xbytes;
///
/// assert_eq!(xbytes!(b"\0\xFF"), [0, 255]);
/// ```
#[proc_macro]
pub fn xbytes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitByteStr);
    obfuscated_bytes(&input.value(), input.span()).into()
}

/// Obfuscates a UTF-8 string literal and returns an owned `CString` for FFI.
/// Interior NUL bytes cause a compile-time error.
///
/// ```
/// use xengine::xcstr;
///
/// assert_eq!(xcstr!("xengine").as_bytes_with_nul(), b"xengine\0");
/// ```
///
/// ```compile_fail
/// xengine::xcstr!("embedded\0NUL");
/// ```
#[proc_macro]
pub fn xcstr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let text = input.value();
    if let Some(position) = text.bytes().position(|byte| byte == 0) {
        return syn::Error::new(input.span(), format!("NUL byte at byte offset {position}"))
            .into_compile_error()
            .into();
    }

    let bytes = obfuscated_bytes(text.as_bytes(), input.span());
    quote! {
        ::std::ffi::CString::new(#bytes).expect("xengine encoded C string must be NUL-free")
    }
    .into()
}

/// Obfuscates a file's contents at compile time and returns an owned `Vec<u8>`.
///
/// Relative paths start at the consuming crate's `src` directory. Cargo tracks
/// the file as a build input.
#[proc_macro]
pub fn xinclude_bytes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    expand_include(&input, IncludeKind::Bytes)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Obfuscates a UTF-8 text file at compile time and returns an owned `String`.
/// Invalid UTF-8 causes a compile-time error. File paths and Cargo rebuild
/// tracking work the same way as in `xinclude_bytes!`.
#[proc_macro]
pub fn xinclude_str(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    expand_include(&input, IncludeKind::Text)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

enum IncludeKind {
    Bytes,
    Text,
}

/// Use an anonymous constant to register the source file in rustc's dep-info.
fn expand_include(input: &LitStr, kind: IncludeKind) -> syn::Result<TokenStream2> {
    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR")
        .ok_or_else(|| syn::Error::new(input.span(), "CARGO_MANIFEST_DIR is unavailable"))?;
    let path = PathBuf::from(manifest_dir).join("src").join(input.value());
    let bytes = fs::read(&path).map_err(|error| {
        syn::Error::new(
            input.span(),
            format!("failed to read '{}': {error}", path.display()),
        )
    })?;
    let tracked_path = path
        .to_str()
        .ok_or_else(|| syn::Error::new(input.span(), "included file path must be valid UTF-8"))?;
    let value = match kind {
        IncludeKind::Bytes => obfuscated_bytes(&bytes, input.span()),
        IncludeKind::Text => {
            std::str::from_utf8(&bytes).map_err(|error| {
                syn::Error::new(
                    input.span(),
                    format!("'{}' is not valid UTF-8: {error}", path.display()),
                )
            })?;
            obfuscated_string(&bytes, input.span())
        }
    };

    Ok(quote! {{
        const _: &[u8] = ::core::include_bytes!(#tracked_path);
        #value
    }})
}

fn obfuscated_bytes(bytes: &[u8], span: proc_macro2::Span) -> TokenStream2 {
    let key: u8 = rand::thread_rng().gen_range(1..=u8::MAX);
    let encrypted: Vec<u8> = bytes.iter().map(|byte| byte ^ key).collect();
    let encrypted = LitByteStr::new(&encrypted, span);

    quote! {{
        let key = ::core::hint::black_box(#key);
        #encrypted
            .iter()
            .map(|byte| byte ^ key)
            .collect::<::std::vec::Vec<u8>>()
    }}
}

/// Inserts the specified number of local assembly branches on x86-64.
///
/// The expansion evaluates to `()`. A count of zero emits no assembly.
#[proc_macro]
pub fn xtrash(input: TokenStream) -> TokenStream {
    let count = parse_macro_input!(input as LitInt);
    let count = match count.base10_parse::<usize>() {
        Ok(count) => count,
        Err(error) => return error.into_compile_error().into(),
    };

    flow_block(count).into()
}

/// Each branch stays within its assembly block and only changes a scratch register and flags.
fn flow_block(count: usize) -> TokenStream2 {
    if count == 0 {
        return quote! { {} };
    }

    let mut rng = rand::thread_rng();
    let mut instructions = Vec::with_capacity(count);
    for _ in 0..count {
        let branch = if rng.gen_bool(0.5) { "jz" } else { "jnz" };
        let value = rng.gen_range(0..=1);
        instructions.push(format!(
            "mov {{scratch:e}}, {value}\ntest {{scratch:e}}, {{scratch:e}}\n{branch} 2f\nnop\n2:"
        ));
    }

    quote! {{
        #[cfg(target_arch = "x86_64")]
        unsafe {
            ::core::arch::asm!(
                #(#instructions,)*
                scratch = out(reg) _,
                options(nomem, nostack),
            );
        }
        #[cfg(not(target_arch = "x86_64"))]
        ::core::compile_error!("xengine control-flow macros require an x86-64 target");
    }}
}

/// Evaluates an expression once and returns its value if it finishes within 10 ms.
///
/// # Panics
///
/// Panics if the expression takes longer than 10 ms, regardless of the cause.
#[proc_macro]
pub fn xanti_dbg(input: TokenStream) -> TokenStream {
    let expression = parse_macro_input!(input as Expr);

    quote! {{
        let start = ::std::time::Instant::now();
        let result = (|| #expression)();
        if start.elapsed() > ::std::time::Duration::from_millis(10) {
            ::core::panic!("xanti_dbg!: expression exceeded 10 ms");
        }
        result
    }}
    .into()
}

/// Inserts the specified number of local assembly branches before each top-level
/// statement in a function on x86-64. A count of zero leaves the body unchanged.
#[proc_macro_attribute]
pub fn xfn(attr: TokenStream, item: TokenStream) -> TokenStream {
    let count = parse_macro_input!(attr as LitInt);
    let count = match count.base10_parse::<usize>() {
        Ok(count) => count,
        Err(error) => return error.into_compile_error().into(),
    };
    let mut function = parse_macro_input!(item as ItemFn);
    if count != 0 {
        let original_statements = std::mem::take(&mut function.block.stmts);
        for statement in original_statements {
            let flow = flow_block(count);
            function.block.stmts.push(parse_quote! { #flow; });
            function.block.stmts.push(statement);
        }
    }

    quote! { #function }.into()
}
