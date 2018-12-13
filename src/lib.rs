use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use failure::{bail, Error, Fail, ResultExt};
use std::{
    io,
    io::{ErrorKind, Read, Write},
};

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
    let clientVersion = stream
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
    }
}

const SERVE_MAGIC_1: u64 = 0x390c9deb;
const SERVE_MAGIC_2: u64 = 0x5452eecb;
// TODO(akavel): use protocol version 0x205
const SERVE_PROTOCOL_VERSION: u64 = 0x204;
