# assert_size

A macro to assert that a type has a specific size at compile time

## Usage

```rust
use assert_size::assert_size;

assert_size!((), 0);
assert_size!(u8, 1);
assert_size!(u16, 2);

struct Foo {
    a: u8, // 1 byte
    // 1 byte padding
    b: u16, // 2 bytes
}

assert_size!(Foo, 4);
```

## License

Licensed under either of

-   Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
-   MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
