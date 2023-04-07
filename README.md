# better-default-derive ![crates.io](https://img.shields.io/crates/v/better-default-derive.svg) ![github actions badge](https://github.com/monadplus/better-default-derive/actions/workflows/ci.yml/badge.svg)

A better [std::default::Default](https://doc.rust-lang.org/nightly/std/default/macro.Default.html) macro.

```toml
[dependencies]
better-default-derive = "0.1.0"
```

## Example
```rust 
use better_default_derive::Default;

#[derive(Debug, PartialEq, Eq, Default)]
enum Either<L, R> {
    Left(L),
    #[default]
    Right(R),
}

fn main() {
    let either: Either<String, u8> = Either::default();
    assert_eq!(either, Either::Right(u8::default()));
}
```

## Supports

- Enums
  - Unit
  - Named
  - Unnamed
  - Generic

## Roadmap

- [ ] Support structs
  - [ ] Unit
  - [ ] Named
  - [ ] Unnamed
  - [ ] Generic
- [ ] Support associated types
