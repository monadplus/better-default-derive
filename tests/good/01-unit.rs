#[derive(Debug, PartialEq, Eq, better_default_derive::Default)]
enum Letters {
    #[default]
    A, 
    B, 
    C,
}

fn main() {
    assert_eq!(Letters::default(), Letters::A);
}
