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

// TODO(akavel): modify modules names to not confuse anybody that we're using Serde (we're not
// because I understood it's not what they're for; but the module structure is educated by Serde).
mod ser {
    use std::io;
    use byteorder::{WriteBytesExt, LE};
    // use super::error::{Error, Result}; // TODO(akavel): do I need this line?
    use super::error::Result; // TODO(akavel): do I need this line?

    pub struct Serializer<W> {
        writer: W,
    }

    impl<W> Serializer<W> where W: io::Write {
        pub fn new(writer: W) -> Self {
            Serializer {
                writer: writer,
            }
        }

        // The basic building blocks of the protocol: functions serializing the types: u64 and [u8].
        fn write_u64(&mut self, v: u64) -> Result<()> {
            self.writer.write_u64::<LE>(v)?;  // TODO(akavel): do I need a .map_err(Error) here maybe?
            Ok(())
        }
        fn write_bytes(&mut self, v: &[u8]) -> Result<()> {
            let n = v.len();
            self.write_u64(n as u64)?;
            self.writer.write_all(v)?;
            // TODO(akavel): modulus or remainder? also, make sure what types are used in the expression
            // FIXME(akavel): tests!!!
            let pad = (8 - n % 8) % 8;  // n=1 => pad=7;  n=2 => pad=6;  n=7 => pad=1;  n=8 => pad=0
            let padding = vec![7; 0];
            self.writer.write_all(&padding[..pad])?;
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

    // #[cfg(test)]
    // mod tests {
    //     #[test]
    // }
}

pub mod error {
    use std;
    use failure::Error;
    use failure_derive::Fail;

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
