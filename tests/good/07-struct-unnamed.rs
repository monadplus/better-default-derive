use better_default_derive::Default;

#[derive(Debug, PartialEq, Eq, Default)]
struct Foo(u8);

#[derive(Debug, PartialEq, Eq, Default)]
struct Bar(Foo, u8);

fn main() {
    assert_eq!(Bar::default(), Bar(Foo(u8::default()), u8::default()));
}
