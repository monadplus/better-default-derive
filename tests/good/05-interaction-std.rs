#[derive(Debug, PartialEq, Eq, std::default::Default)]
struct Foo {
    foo: u8,
    bar: Bar,
}
#[derive(Debug, PartialEq, Eq, better_default_derive::Default)]
enum Bar {
    #[default]
    Bar(u8),
    Baz(u8),
}

fn main() {}
