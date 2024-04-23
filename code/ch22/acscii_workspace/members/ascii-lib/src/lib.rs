mod my_ascii {

    use std::error;
    use std::fmt;

    // When conversion fails, we give back the vector we couldn't convert.
    #[derive(Debug, Eq, PartialEq)]
    pub struct NotAsciiError(pub Vec<u8>);

    impl fmt::Display for NotAsciiError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Found non-ASCII bytes: {:?}", self.0)
        }
    }

    impl error::Error for NotAsciiError {}

    // Safe, efficient conversion, implemented using unsafe code.
    impl From<Ascii> for String {
        fn from(ascii: Ascii) -> String {
            // If this module has no bugs, this is safe, because
            // well-formed ASCII text is also well-formed UTF-8.
            unsafe { String::from_utf8_unchecked(ascii.0) }
        }
    }

    // Define a new type to represent the result of ASCII conversion
    pub struct AsciiResult(pub Result<Ascii, NotAsciiError>);

    impl From<AsciiResult> for String {
        fn from(result: AsciiResult) -> Self {
            match result.0 {
                Ok(ascii) => ascii.into(),
                Err(err) => format!("{}", err), // Convert error to string
            }
        }
    }

    /// An ASCII-encoded string.
    #[derive(Debug, Eq, PartialEq)]
    pub struct Ascii(
        // This must hold only well-formed ASCII text:
        // bytes from `0` to `0x7f`.
        Vec<u8>,
    );

    impl Ascii {
        /// Create an `Ascii` from the ASCII text in `bytes`. Return a
        /// `NotAsciiError` error if `bytes` contains any non-ASCII
        /// characters.
        pub fn from_bytes(bytes: Vec<u8>) -> Result<Ascii, NotAsciiError> {
            if bytes.iter().any(|&byte| !byte.is_ascii()) {
                return Err(NotAsciiError(bytes));
            }
            Ok(Ascii(bytes))
        }
    }

    #[cfg(test)]
    mod tests {
        // use super::*;

        #[test]
        fn good_ascii() {
            use crate::my_ascii::Ascii;

            let bytes: Vec<u8> = b"ASCII and ye shall receive".to_vec();

            // This call entails no allocation or text copies, just a scan.
            let ascii: Ascii = Ascii::from_bytes(bytes).unwrap(); // We know these chosen bytes are ok.

            // This call is zero-cost: no allocation, copies, or scans.
            let string = String::from(ascii);

            assert_eq!(string, "ASCII and ye shall receive");
        }
    }

    
#[test]
fn bad_ascii() {
    use crate::my_ascii::Ascii;

    // Imagine that this vector is the result of some complicated process
    // that we expected to produce ASCII. Something went wrong!
    let bytes = vec![0xf7, 0xbf, 0xbf, 0xbf];

    let ascii  = AsciiResult(Ascii::from_bytes(bytes));

    let bogus: String = ascii.into();

    // `bogus` now holds ill-formed UTF-8. Parsing its first character produces
    // a `char` that is not a valid Unicode code point. That's undefined
    // behavior, so the language doesn't say how this assertion should behave.
    assert_ne!(bogus.chars().next().unwrap() as u32, 0x1fffff);
}
}
