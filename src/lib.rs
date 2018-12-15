use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use failure::{bail, Error, ResultExt};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::{
    io::{ErrorKind, Read, Write},
};

mod serde_nix;

// Based on NIX/src/nix-store/nix-store.cc, opServe()
// Other references:
// - NIX/src/libstore/legacy-ssh-store.cc
pub fn serve(stream: &mut (impl Read + Write)) -> Result<(), Error> {
    // TODO(akavel): add serde helper for reading u64 always as LE, reading strings, etc.

    // Exchange initial greeting.
    let magic = stream
        .read_u64::<LE>()
        .context("cannot read 'hello' magic number")?;
    if magic != SERVE_MAGIC_1 {
        bail!("protocol mismatch");
    }
    stream.write_u64::<LE>(SERVE_MAGIC_2)?;
    stream.write_u64::<LE>(SERVE_PROTOCOL_VERSION)?;
    let _clientVersion = stream
        .read_u64::<LE>()
        .context("cannot read client version")?;

    // Handle commands.
    loop {
        let cmd = match stream.read_u64::<LE>() {
            Ok(x) => x,
            Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => {
                return Ok(());
            }
            Err(e) => return Err(Error::from(e)),
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
