// Helpers for serializing/deserializing basic data types to/from Nix communication protocol
// format.
// Based on:
//  - https://gist.github.com/jbeda/5c79d2b1434f0018d693 (copied from Figure 5.2 in Eelco's thesis
//    at http://nixos.org/~eelco/pubs/phd-thesis.pdf)
//  - https://github.com/NixOS/nix/blob/7b0b349085cf7cddb61e49b809d2be7ac28fe53e/src/libutil/serialise.cc

// TODO(akavel): publish what should be published:
// pub use de::{from_reader, Deserializer};
// pub use error::{Error, Result};
// pub use ser::{to_writer, Serializer};

// mod de;

// Internal function, used to calculate length of 0-padding for byte slices in Nix protocol.
// n=1 => pad=7;  n=2 => pad=6;  n=7 => pad=1;  n=8 => pad=0
// FIXME(akavel): should below fn take u64 instead?
const fn pad(n: usize) -> usize {
    (8 - n % 8) % 8
}

// Based on source code of the std::fmt::format! macro.
// TODO(akavel): can this be simplified? feels much overcomplicated, am I missing something?
// TODO(akavel): this is internal; I'd prefer this further down, but macros must be defined before
// used; TODO: move to separate file
macro_rules! protocol_error {
    ($($arg:tt)*) => (
        Err(failure::Error::from(ProtocolError::Message {
            msg: format!($($arg)*)
        }))
    )
}

// TODO(akavel): modify modules names to not confuse anybody that we're using Serde (we're not
// because I understood it's not what they're for; but the module structure is educated by Serde).
mod ser {
    use byteorder::{WriteBytesExt, LE};
    use std::io;
    // use super::error::{Error, Result}; // TODO(akavel): do I need this line?
    use super::error::Result; // TODO(akavel): do I need this line?

    pub struct Serializer<W> {
        writer: W,
    }

    impl<W> Serializer<W>
    where
        W: io::Write,
    {
        pub fn new(writer: W) -> Self {
            Serializer { writer: writer }
        }

        // The basic building blocks of the protocol: functions serializing the types: u64 and [u8].
        fn write_u64(&mut self, v: u64) -> Result<()> {
            self.writer.write_u64::<LE>(v)?; // TODO(akavel): do I need a .map_err(Error) here maybe?
            Ok(())
        }
        fn write_bytes(&mut self, v: &[u8]) -> Result<()> {
            let n = v.len();
            self.write_u64(n as u64)?;
            self.writer.write_all(v)?;
            // TODO(akavel): modulus or remainder? also, make sure what types are used in the expression
            // FIXME(akavel): tests!!!
            let padding = [0; 7];
            self.writer.write_all(&padding[..super::pad(n)])?;
            Ok(())
        }
        // Helper functions converting various types into a single u64 or [u8].
        fn write_bool(&mut self, v: bool) -> Result<()> {
            self.write_u64(if v { 1 } else { 0 })
        }
        // fn serialize_str(self, v: &str) -> Result<()> {
        //     // FIXME(akavel): super important: what encoding is used by Nix for strings in the protocol?
        //     self.serialize_bytes(
    }

    #[cfg(test)]
    mod tests {
        use hex_literal::{hex, hex_impl};

        // NOTE(akavel): macro-driven tests, based on:
        // https://github.com/coriolinus/exercism_rust/commit/e94389860c7126f5c562cd415d51589bf035d9df
        // https://github.com/BurntSushi/fst/blob/715919a1bf658501f9028dbc5c3b7ebb5a508ea2/src/raw/tests.rs#L111-L158
        macro_rules! test {
            ($method:ident, $testname:ident, $input:expr, $expect:expr) => {
                #[test]
                fn $testname() {
                    let mut buf = std::vec::Vec::new();
                    super::Serializer::new(&mut buf).$method($input).unwrap();
                    assert_eq!(buf, $expect);
                }
            };
        }

        #[rustfmt::skip] test!(write_bytes, write_bytes_len1, b"A", hex!("
            01 00 00 00  00 00 00 00
            41 00 00 00  00 00 00 00
        "));
        #[rustfmt::skip] test!(write_bytes, write_bytes_len2, b"AB", hex!("
            02 00 00 00  00 00 00 00
            41 42 00 00  00 00 00 00
        "));
        #[rustfmt::skip] test!(write_bytes, write_bytes_len8, b"AAAABBBB", hex!("
            08 00 00 00  00 00 00 00
            41 41 41 41  42 42 42 42
        "));
        #[rustfmt::skip] test!(write_bytes, write_bytes_len9, b"AAAABBBBC", hex!("
            09 00 00 00  00 00 00 00
            41 41 41 41  42 42 42 42
            43 00 00 00  00 00 00 00
        "));
        // TODO(akavel): should 0-byte strings be supported? or should it be an error?
        #[rustfmt::skip] test!(write_bytes, write_bytes_len0, b"", hex!("
            00 00 00 00  00 00 00 00
        "));

        #[rustfmt::skip] test!(write_bool, write_bool_true,  true,  hex!("01 00 00 00  00 00 00 00"));
        #[rustfmt::skip] test!(write_bool, write_bool_false, false, hex!("00 00 00 00  00 00 00 00"));
    }
}

pub mod de {
    use super::error::{ProtocolError, Result};
    use byteorder::{ReadBytesExt, LE};
    use failure;
    use std::io;

    pub struct Deserializer<R> {
        reader: R,
    }

    impl<R> Deserializer<R>
    where
        R: io::Read,
    {
        fn read_u64(&mut self) -> Result<u64> {
            // TODO(akavel): is there a way to make below map_err unnecessary, or simplify it?
            self.reader
                .read_u64::<LE>()
                .map_err(|e| failure::Error::from(e))
        }

        // fn read_bytes(&mut self) -> Result<&[u8]> {
        //     let n = self.read_u64()?;
        //     // TODO(akavel): is there a way to simplify/shorten below lines?
        //     // FIXME(akavel): limit n somehow??? now it can be >6 ZiB (a.k.a. 6 mln TiB) or
        //     // something! Maybe instead return (n, io::Read) or something?
        //     let mut buf = vec![0; n];
        //     self.reader.read_exact(&mut buf)?;
        //     let mut padding = [0; 7];
        //     self.reader.read_exact(&mut padding[..super::pad(n as usize)])?;
        //     Ok(&buf)
        // }

        //
        // Helper functions, converting the basic protocol atoms into other types
        //

        fn read_bool(&mut self) -> Result<bool> {
            // TODO(akavel): or should it be ... == 1 ?
            Ok(self.read_u64()? != 0)
        }

        // Read a string composed only of printable 7-bit ASCII characters and space, with a
        // maximum length as specified. If longer string was found, or non-fitting bytes, return
        // error.
        fn read_str_ascii(&mut self, max: u64) -> Result<String> {
            let n = self.read_u64()?;
            if n > max {
                // FIXME(akavel): add offset info in the error
                return protocol_error!("string too long, expected max {} bytes, got {}", max, n);
            }
            let mut buf = vec![0; n as usize];
            self.reader.read_exact(&mut buf)?;
            let mut padding = [0; 7];
            self.reader
                .read_exact(&mut padding[..super::pad(n as usize)])?;
            // Verify string contents
            fn is_ok(b: u8) -> bool {
                (b'a' <= b && b <= b'z')
                    || (b'A' <= b && b <= b'Z')
                    || (b'0' <= b && b <= b'9')
                    || b" `~!@#$%^&*()_+-=[]{};':\"\\|,./<>?".contains(&b)
            }
            if let Some(bad_byte) = buf.iter().find(|&&b| !is_ok(b)) {
                return protocol_error!(
                    "unexpected byte '{}' (hex {}) in string: {:x?}",
                    bad_byte,
                    *bad_byte as char,
                    buf.as_slice()
                );
            }
            // TODO(akavel): optimize this with from_utf8_unchecked?
            Ok(String::from_utf8(buf)?)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Deserializer;
        use hex_literal::{hex, hex_impl};

        #[test]
        #[rustfmt::skip]
        fn test_read_bool() {
            assert_eq!(false, Deserializer { reader: &hex!("00 00 00 00  00 00 00 00")[..] }.read_bool().unwrap());
            assert_eq!(true,  Deserializer { reader: &hex!("01 00 00 00  00 00 00 00")[..] }.read_bool().unwrap());
            assert_eq!(true,  Deserializer { reader: &hex!("ff ff ff ff  ff ff ff ff")[..] }.read_bool().unwrap());
        }

        #[test]
        fn test_read_sample_string() {
            let buf = [
                hex!("3a 00 00 00 00 00 00 00  2f 6e 69 78 2f 73 74 6f"), // :......./nix/sto |
                hex!("72 65 2f 32 6b 63 72 6a  31 6b 73 64 32 61 31 34"), // re/2kcrj1ksd2a14 |
                hex!("62 6d 35 73 6b 79 31 38  32 66 76 32 78 77 66 68"), // bm5sky182fv2xwfh |
                hex!("66 61 70 2d 67 6c 69 62  63 2d 32 2e 32 36 2d 31"), // fap-glibc-2.26-1 |
                hex!("33 31 00 00 00 00 00 00  38 00 00 00 00 00 00 00"), // 31......8....... |
            ].concat();
            let mut deserializer = Deserializer { reader: &buf[..] };
            assert_eq!(
                deserializer.read_str_ascii(100).unwrap(),
                "/nix/store/2kcrj1ksd2a14bm5sky182fv2xwfhfap-glibc-2.26-131"
            );
        }
    }
}

pub mod error {
    use failure::Error;
    use failure_derive::Fail;
    use std;

    pub type Result<T> = std::result::Result<T, Error>;

    // TODO(akavel): add offset info for de, [LATER] add key info for ser
    // TODO(akavel): understand what's going on here; generally copied some minimum from the serde guide
    #[derive(Debug, Fail)]
    pub enum ProtocolError {
        #[fail(display = "{}", msg)]
        Message { msg: String },
        #[fail(display = "unexpected end of input")]
        UnexpectedEof,
    }

    // // Some boilerplate (?)
    // impl std::fmt::Display for Error {
    //     fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
    //         formatter.write_str(std::error::Error::description(self))
    //     }
    // }

    // // Error messages for the enum variants
    // impl std::error::Error for Error {
    //     fn description(&self) -> &str {
    //         match *self {
    //             Error::Message(ref msg) => msg,
    //             Error::UnexpectedEof => "unexpected end of input",
    //         }
    //     }
    // }

    // impl std::convert::From<std::io::Error> for Error {
    //     fn from(e: std::io::Error) -> Error {
    //         Error::Io(e)
    //     }
    // }
}
