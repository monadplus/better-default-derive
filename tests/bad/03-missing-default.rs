use better_default_derive::Default;

#[derive(Debug, PartialEq, Eq)]
struct Foo(u8);

#[derive(Debug, PartialEq, Eq, Default)]
struct Bar {
    foo: Foo, 
    bar: u8,
}

fn main() {}
