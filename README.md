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
