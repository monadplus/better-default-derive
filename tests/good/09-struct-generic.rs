use better_default_derive::Default;

#[derive(Debug, PartialEq, Eq, Default)]
struct Both<L, R> {
    left: L,
    right: R,
}

fn main() {
    let result: Both<String, u8> = Both::default();
    let expected: Both<String, u8> = Both {
        left: String::default(),
        right: u8::default(),
    };
    assert_eq!(result, expected);
}
