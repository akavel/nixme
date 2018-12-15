// Helpers for serializing/deserializing basic data types to/from Nix communication protocol
// format.
// Based on:
//  - https://gist.github.com/jbeda/5c79d2b1434f0018d693 (copied from Figure 5.2 in Eelco's thesis
//    at http://nixos.org/~eelco/pubs/phd-thesis.pdf)
//  - https://github.com/NixOS/nix/blob/7b0b349085cf7cddb61e49b809d2be7ac28fe53e/src/libutil/serialise.cc
//  - Serde guide on writing a custom data format: https://serde.rs/data-format.html
//  - https://github.com/serde-rs/json/tree/master/src/

// use serde_derive::{Deserialize, Serialize};

pub use de::{from_reader, Deserializer};
pub use error::{Error, Result};
pub use ser::{to_writer, Serializer};

mod de;
mod ser;

mod ser {
    use serde::ser::{self, Serialize};
    use byteorder::{WriteBytesExt, LE};
    use error::{Error, Result}; // TODO(akavel): do I need this line?

    pub struct Serializer<W> {
        writer: W,
    }

    pub fn to_writer<W, T>(writer: W, value: &T) -> Result<()>
    where
        W: io::Write,
        T: Serialize,
    {
        let mut ser = Serializer {
            writer: writer,
        };
        value.serialize(&mut ser)?;
        Ok(())
    }

    impl<'a, W> ser::Serializer for &'a mut Serializer<W>
    where
        W: io::Write,
    {
        // Boilerplate for serde
        type Ok = ();
        type Error = Error;
        // TODO(akavel): Associated types for keeping extra state when serializing non-primitive
        // (a.k.a. compound) data structures (such as struct, map, ...), for serde. If no extra
        // state is needed, Self can be used.
        // type SerializeSeq = Self;
        // type SerializeTuple = Self;
        // type SerializeTupleStruct = Self;
        // type SerializeTupleTupleVariant = Self;
        // type SerializeMap = Self;
        // type SerializeStruct = Self;
        // type SerializeStructVariant = Self;

        // The basic building blocks of the protocol: functions serializing the types: u64 and [u8].
        fn serialize_u64(self, v: u64) -> Result<()> {
            self.writer.write_u64<LE>(v)  // TODO(akavel): do I need a .map_err(Error) here maybe?
        }
        fn serialize_bytes(self, v: &[u8]) -> Result<()> {
            let n = v.len();
            self.serialize_u64(n)?;
            self.writer.write_all(v)?;
            // TODO(akavel): modulus or remainder? also, make sure what types are used in the expression
            // FIXME(akavel): tests!!!
            let pad = (8 - n % 8) % 8;  // n=1 => pad=7;  n=2 => pad=6;  n=7 => pad=1;  n=8 => pad=0
            let padding = vec![7; 0];
            self.writer.write_all(&padding[..pad])
        }
        // Helper functions converting various types into a single u64 or [u8].
        fn serialize_bool(self, v: bool) -> Result<()> {
            self.serialize_u64(if v { 1 } else { 0 })
        }
        // fn serialize_str(self, v: &str) -> Result<()> {
        //     // FIXME(akavel): super important: what encoding is used by Nix for strings in the protocol?
        //     self.serialize_bytes(
}

mod error {
    use std;
    use serde::{de, ser};

    pub type Result<T> = std::result::Result<T, Error>;

    // TODO(akavel): add offset info for de, [LATER] add key info for ser
    // TODO(akavel): understand what's going on here; generally copied some minimum from the serde guide
    #[derive(Clone, Debug, PartialEq)]
    pub enum Error {
        Message(string),
        UnexpectedEof,
    }

    // Some boilerplate (?)
    impl ser::Error for Error {
        fn custom<T: Display>(msg: T) -> Self {
            Error::Message(msg.to_string())
        }
    }
    impl de::Error for Error {
        fn custom<T: Display>(msg: T) -> Self {
            Error::Message(msg.to_string())
        }
    }
    impl Display for Error {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(std::error::Error::description(self))
        }
    }

    // Error messages for the enum variants
    impl std::error::Error for Error {
        fn description(&self) -> &str {
            match *self {
                Error::Message(ref msg) => msg,
                Error::UnexpectedEof => "unexpected end of input",
            }
        }
    }
}
