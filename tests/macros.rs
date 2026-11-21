use xengine::{xanti_dbg, xfn, xinclude_bytes, xstr, xtrash};

#[test]
fn string_literals_round_trip() {
    assert_eq!(xstr!(""), "");
    assert_eq!(xstr!("Привет, café, 🦀!"), "Привет, café, 🦀!");
    assert_eq!(xstr!("first\0\nsecond"), "first\0\nsecond");
    assert_eq!(xstr!(r#"a raw "string""#), r#"a raw "string""#);
}

#[test]
fn generated_executable_omits_contiguous_plaintext() {
    let string: String = xstr!("xengine-string-a6f91bf542");
    let bytes: Vec<u8> = xinclude_bytes!("../tests/fixtures/embedded data.bin");
    let string_marker = reverse(b"245fb19f6a-gnirts-enignex");
    let file_marker = reverse(b"f56c38a625-erutxif-ENIGNEX");
    assert_eq!(string.as_bytes(), string_marker);

    let mut expected_bytes = vec![0, 128, 255];
    expected_bytes.extend_from_slice(&file_marker);
    expected_bytes.extend_from_slice(&[10, 13, 0, 254]);
    assert_eq!(bytes, expected_bytes);

    let executable = std::fs::read(std::env::current_exe().unwrap()).unwrap();
    for marker in [&string_marker, &file_marker] {
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
