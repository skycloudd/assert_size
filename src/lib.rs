#[macro_export]
macro_rules! assert_size {
    ($size:expr, $t:ty) => {
        const _: [(); $size] = [(); core::mem::size_of::<$t>()];
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zst() {
        assert_size!(0, ()); // 0 bytes
    }

    #[test]
    fn bool() {
        assert_size!(1, bool); // 1 byte
    }

    #[test]
    fn array() {
        assert_size!(3, [u8; 3]); // 3 * u8 = 3 bytes
    }

    #[test]
    fn struct_() {
        #[allow(dead_code)]
        struct S {
            a: u8, // 1 byte
            // 1 byte padding
            b: u16, // 2 bytes
        }

        assert_size!(4, S); // 2 + 1 + 1 (padding)
    }

    #[test]
    fn enum_() {
        #[allow(dead_code)]
        enum E {
            A,      // 1 byte  (discriminant)
            B(u8),  // 2 bytes (discriminant + u8)
            C(u16), // 4 bytes (discriminant + padding + u16)
        }

        assert_size!(4, E); // 4 bytes
    }
}
