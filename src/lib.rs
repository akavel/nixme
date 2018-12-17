use crate::serde_nix::de::Deserializer;
use crate::serde_nix::ser::Serializer;
use failure::{bail, Error, ResultExt};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::io::{ErrorKind, Read, Write};

mod serde_nix;

// Based on NIX/src/nix-store/nix-store.cc, opServe()
// Other references:
// - NIX/src/libstore/legacy-ssh-store.cc
pub fn serve(mut stream: &mut (impl Read + Write)) -> Result<(), Error> {
    let mut ser = Serializer { writer: &mut stream };
    let mut de = Deserializer { reader: &mut stream };
    // Exchange initial greeting.
    let magic = de.read_u64().context("cannot read 'hello' magic number")?;
    if magic != SERVE_MAGIC_1 {
        bail!("protocol mismatch");
    }
    ser.write_u64(SERVE_MAGIC_2)?;
    ser.write_u64(SERVE_PROTOCOL_VERSION)?;
    let _clientVersion = de.read_u64().context("cannot read client version")?;

    // Handle commands.
    loop {
        let cmd = match de.read_u64() {
            Ok(x) => x,
            Err(e) => {
                if let Some(ref cause) = e.downcast_ref::<std::io::Error>() {
                    if cause.kind() == ErrorKind::UnexpectedEof {
                        return Ok(());
                    }
                }
                return Err(e);
                // return Err(Error::from(e))
            }
            // Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => {
            //     return Ok(());
            // }
            // Err(e) => return Err(Error::from(e)),
        };
        match FromPrimitive::from_u64(cmd) {
            Some(Command::QueryValidPaths) => {
                println!("query v.p.!");
            }
            _ => {
                panic!("unknown cmd {}", cmd);
            }
        }
    }
}

const SERVE_MAGIC_1: u64 = 0x390c9deb;
const SERVE_MAGIC_2: u64 = 0x5452eecb;
// TODO(akavel): use protocol version 0x205
const SERVE_PROTOCOL_VERSION: u64 = 0x204;

#[derive(FromPrimitive)]
enum Command {
    QueryValidPaths = 1,
}
