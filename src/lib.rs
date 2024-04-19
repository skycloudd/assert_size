#![no_std]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

/// Asserts that the size of a type is equal to a given value.
///
/// # Examples
///
/// ```
/// use assert_size::assert_size;
///
/// assert_size!((), 0); // 0 bytes
/// assert_size!(bool, 1); // 1 byte
/// assert_size!([u8; 3], 3); // 3 bytes
/// ```
#[macro_export]
macro_rules! assert_size {
    ($t:ty, $size:expr) => {
        const _: [(); $size] = [(); core::mem::size_of::<$t>()];
    };
}

#[cfg(test)]
mod tests {
    use super::assert_size;

    #[test]
    const fn zst() {
        assert_size!((), 0); // 0 bytes
    }

    #[test]
    const fn bool() {
        assert_size!(bool, 1); // 1 byte
    }

    #[test]
    const fn array() {
        assert_size!([u8; 3], 3); // 3 * u8 = 3 bytes
    }

    #[test]
    const fn struct_() {
        #[allow(dead_code)]
        struct S {
            a: u8, // 1 byte
            // 1 byte padding
            b: u16, // 2 bytes
        }

        assert_size!(S, 4); // 2 + 1 + 1 (padding)
    }

    #[test]
    const fn enum_() {
        #[allow(dead_code)]
        enum E {
            A,      // 1 byte  (discriminant)
            B(u8),  // 2 bytes (discriminant + u8)
            C(u16), // 4 bytes (discriminant + padding + u16)
        }

        assert_size!(E, 4); // 4 bytes
    }
}
