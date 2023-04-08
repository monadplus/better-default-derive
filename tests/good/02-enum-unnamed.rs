#[derive(Debug, PartialEq, Eq, Default)]
struct A {
    a: u8,
    b: String,
}

#[derive(Debug, PartialEq, Eq)]
struct B {
    a: u8
}

#[derive(Debug, PartialEq, Eq, better_default_derive::Default)]
enum Letters {
    #[default]
    A(A, u8),
    B(B)
}

fn main() {
    assert_eq!(Letters::default(), Letters::A(A::default(), u8::default()));
}
