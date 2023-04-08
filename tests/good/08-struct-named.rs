use better_default_derive::Default;

#[derive(Debug, PartialEq, Eq, Default)]
struct Foo(u8);

#[derive(Debug, PartialEq, Eq, Default)]
struct Bar {
    foo: Foo, 
    bar: u8,
}

fn main() {
    let expected = Bar {
        foo: Foo(u8::default()),
        bar: u8::default(),
    };
    assert_eq!(Bar::default(), expected);
}
