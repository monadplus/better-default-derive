#[derive(Debug, PartialEq, Eq, better_default_derive::Default)]
struct Foo(u8);

#[derive(Debug, PartialEq, Eq, better_default_derive::Default)]
struct Bar(Foo);

fn main() {
    assert_eq!(Bar::default(), Bar(Foo(u8::default())));
}
