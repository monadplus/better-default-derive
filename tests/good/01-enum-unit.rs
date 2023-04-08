use better_default_derive::Default;

#[derive(Debug, PartialEq, Eq, Default)]
enum Letters {
    #[default]
    A, 
    B, 
    C,
}

fn main() {
    assert_eq!(Letters::default(), Letters::A);
}
