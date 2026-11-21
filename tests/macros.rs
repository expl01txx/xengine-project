use xengine::{xanti_dbg, xbytes, xcstr, xfn, xinclude_bytes, xinclude_str, xstr, xtrash};

#[test]
fn string_literals_round_trip() {
    assert_eq!(xstr!(""), "");
    assert_eq!(xstr!("Привет, café, 🦀!"), "Привет, café, 🦀!");
    assert_eq!(xstr!("first\0\nsecond"), "first\0\nsecond");
    assert_eq!(xstr!(r#"a raw "string""#), r#"a raw "string""#);
}

#[test]
fn byte_string_literals_round_trip() {
    assert!(xbytes!(b"").is_empty());
    assert_eq!(xbytes!(b"\x00\x80\xff"), [0, 128, 255]);
    assert_eq!(xbytes!(br"raw\bytes"), br"raw\bytes");
}

#[test]
fn c_string_literals_have_one_trailing_nul() {
    assert_eq!(xcstr!("").as_bytes_with_nul(), b"\0");
    assert_eq!(xcstr!("café").as_bytes_with_nul(), "café\0".as_bytes());
}

#[test]
fn generated_executable_omits_contiguous_plaintext() {
    let string: String = xstr!("xengine-string-a6f91bf542");
    let file_bytes: Vec<u8> = xinclude_bytes!("../tests/fixtures/embedded data.bin");
    let text: String = xinclude_str!("../tests/fixtures/message.txt");
    let c_string: std::ffi::CString = xcstr!("xengine-cstr-184c8df72b");
    let literal_bytes: Vec<u8> = xbytes!(b"xengine-bytes-fd56422a01");
    let string_marker = reverse(b"245fb19f6a-gnirts-enignex");
    let file_marker = reverse(b"f56c38a625-erutxif-ENIGNEX");
    let text_marker = reverse(b"a0fe252d4a-txet-enignex");
    let c_string_marker = reverse(b"b27fd8c481-rtsc-enignex");
    let literal_bytes_marker = reverse(b"10a22465df-setyb-enignex");
    assert_eq!(string.as_bytes(), string_marker);
    assert_eq!(c_string.as_bytes(), c_string_marker);
    assert_eq!(literal_bytes, literal_bytes_marker);

    let (text_prefix, text_suffix) = text.as_bytes().split_at(text_marker.len());
    assert_eq!(text_prefix, text_marker);
    assert_eq!(text_suffix, " café 🦀".as_bytes());

    let mut expected_bytes = vec![0, 128, 255];
    expected_bytes.extend_from_slice(&file_marker);
    expected_bytes.extend_from_slice(&[10, 13, 0, 254]);
    assert_eq!(file_bytes, expected_bytes);

    let executable = std::fs::read(std::env::current_exe().unwrap()).unwrap();
    for marker in [
        &string_marker,
        &file_marker,
        &text_marker,
        &c_string_marker,
        &literal_bytes_marker,
    ] {
        assert!(!executable
            .windows(marker.len())
            .any(|window| window == marker));
    }
}

fn reverse(bytes: &[u8]) -> Vec<u8> {
    std::hint::black_box(bytes).iter().rev().copied().collect()
}

#[test]
fn empty_files_round_trip() {
    assert!(xinclude_bytes!("../tests/fixtures/empty.bin").is_empty());
    assert!(xinclude_str!("../tests/fixtures/empty.bin").is_empty());
}

#[test]
fn timed_expression_returns_its_result_and_runs_once() {
    let mut calls = 0;
    let result = xanti_dbg!({
        calls += 1;
        42
    });

    assert_eq!((result, calls), (42, 1));
}

#[test]
#[should_panic(expected = "xanti_dbg!: expression exceeded 10 ms")]
fn timed_expression_panics_after_threshold() {
    xanti_dbg!(std::thread::sleep(std::time::Duration::from_millis(30)));
}

#[xfn(0)]
const fn zero_count_works_in_const_context() -> u8 {
    xtrash!(0);
    42
}

#[test]
fn zero_count_leaves_const_evaluation_available() {
    const VALUE: u8 = zero_count_works_in_const_context();
    assert_eq!(VALUE, 42);
}

#[cfg(target_arch = "x86_64")]
mod flow {
    use super::{xfn, xtrash};

    #[xfn(16)]
    #[test]
    fn instrumentation_preserves_function_attributes_and_results() {
        let value = std::hint::black_box(10);
        assert_eq!(calculate(Some(value)), 21);
        assert_eq!(calculate(None), 0);
    }

    #[inline(always)]
    #[xfn(32)]
    fn calculate(value: Option<u32>) -> u32 {
        let Some(value) = value else { return 0 };
        xtrash!(32);
        value * 2 + 1
    }

    #[test]
    fn standalone_macro_uses_local_labels_in_each_expansion() {
        let mut values = vec![1, 2, 3];
        for value in &mut values {
            xtrash!(64);
            *value += 1;
        }
        xtrash!(64);
        assert_eq!(values, [2, 3, 4]);
    }

    #[test]
    fn qualified_attribute_works_without_macro_imports() {
        mod downstream {
            #[xengine::xfn(8)]
            pub fn increment(value: u32) -> u32 {
                value + 1
            }
        }

        assert_eq!(downstream::increment(41), 42);
    }
}
