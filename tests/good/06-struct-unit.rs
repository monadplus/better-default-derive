use better_default_derive::Default;

#[derive(Debug, PartialEq, Eq, Default)]
struct Bar;

fn main() {
    assert_eq!(Bar::default(), Bar);
}
