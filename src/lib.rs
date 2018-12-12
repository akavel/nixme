use std::io::{Read, Write};
use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use failure::{ResultExt, Error, bail};

// Based on NIX/src/nix-store/nix-store.cc, opServe()
// Other references:
// - NIX/src/libstore/legacy-ssh-store.cc
pub fn serve(stream: &mut (impl Read + Write)) -> Result<(), Error> {
    // TODO(akavel): add serde helper for reading u32 always as LE, reading strings, etc.

    // Exchange initial greeting.
    let magic = stream.read_u32::<LE>().context("cannot read 'hello' magic number")?;
    if magic != SERVE_MAGIC_1 {
        bail!("protocol mismatch");
    }
    stream.write_u32::<LE>(SERVE_MAGIC_2)?;
    stream.write_u32::<LE>(SERVE_PROTOCOL_VERSION)?;
    Ok(())
}

const SERVE_MAGIC_1: u32 = 0x390c9deb;
const SERVE_MAGIC_2: u32 = 0x5452eecb;
const SERVE_PROTOCOL_VERSION: u32 = 0x205;

