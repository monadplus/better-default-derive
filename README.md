# better-default-derive [![crates.io](https://img.shields.io/crates/v/better-default-derive.svg)](https://crates.io/crates/better-default-derive) [![github actions badge](https://github.com/monadplus/better-default-derive/actions/workflows/ci.yml/badge.svg)](https://github.com/monadplus/better-default-derive/actions/workflows/ci.yml)

A better [std::default::Default](https://doc.rust-lang.org/nightly/std/default/macro.Default.html) macro.

Supports:
- Struct
  - Unit
  - Named
  - Unnamed
  - Generic
- Enum
  - Unit
  - Named
  - Unnamed
  - Generic

## Usage

```toml
[dependencies]
better-default-derive = "0.1.0"
```

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

## Roadmap

- [ ] Better error message when a field is missing the `Default` instance.
- [ ] Support union types
- [ ] Support associated types

  ```rust
  use better_default_derive::Default;

  pub trait Trait {
      type Value;
  }

  #[derive(Default)]
  pub struct Field<T: Trait> {
      values: Vec<T::Value>,
  }
  ```
