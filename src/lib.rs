use failure::{bail, Error, ResultExt};
use log::debug;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::io::{ErrorKind, Read, Write};

use crate::stream::Stream;
pub mod nar;
pub mod stream;

// TODO: test 'serve()' for simplified scenario with just cmd 2

// Based on NIX/src/nix-store/nix-store.cc, opServe()
// Other references:
// - NIX/src/libstore/legacy-ssh-store.cc
pub fn serve(store: &mut dyn Store, stream: &mut (impl Read + Write)) -> Result<(), Error> {
    let mut stream = Stream::new(stream);
    // Exchange initial greeting.
    let magic = stream
        .read_u64()
        .context("cannot read 'hello' magic number")?;
    if magic != SERVE_MAGIC_1 {
        debug!("no SERVE_MAGIC_1");
        bail!("protocol mismatch");
    }
    debug!("got SERVE_MAGIC_1");
    stream.write_u64(SERVE_MAGIC_2)?;
    stream.write_u64(SERVE_PROTOCOL_VERSION)?;
    debug!("wrote SERVE_MAGIC_2");
    let _client_version = stream.read_u64().context("cannot read client version")?;

    // Handle commands.
    loop {
        let cmd = match stream.read_u64() {
            Ok(x) => x,
            Err(e) => {
                // TODO(akavel): put below block in helper func, then call it from guard expression
                if let Some(ref cause) = e.downcast_ref::<std::io::Error>() {
                    if cause.kind() == ErrorKind::UnexpectedEof {
                        return Ok(());
                    }
                }
                return Err(e);
            }
        };
        match FromPrimitive::from_u64(cmd) {
            Some(Command::QueryValidPaths) => {
                debug!("cmd 1");
                let _lock = stream.read_bool()?; // TODO[LATER]: implement `lock` handling
                let _substitute = stream.read_bool()?; // TODO[LATER]: implement `substitute` handling
                let paths = stream.read_strings_ascii(100, 300)?;
                let response = store.query_valid_paths(&mut paths.iter().map(|s| &**s));
                stream.write_strings(&response)?;
            }
            Some(Command::QueryPathInfos) => {
                debug!("cmd 2");
                let _paths = stream.read_strings_ascii(100, 300)?;
                // TODO(akavel): do we need to implement this, or is it ok to just fake it?
                stream.write_u64(0)?;
            }
            Some(Command::ImportPaths) => {
                debug!("cmd 4");
                loop {
                    let next = stream.read_u64()?;
                    if next == 0 {
                        break;
                    } else if next != 1 {
                        bail!("input doesn't look like something created by 'nix-store --export'");
                    }
                    let mut handler = NopHandler {};
                    nar::parse(&mut stream, &mut handler)?;
                }
            }
            _ => {
                panic!("unknown cmd {}", cmd);
            }
        }
    }
}

const SERVE_MAGIC_1: u64 = 0x390c_9deb;
const SERVE_MAGIC_2: u64 = 0x5452_eecb;
// TODO(akavel): use protocol version 0x205
const SERVE_PROTOCOL_VERSION: u64 = 0x204;

#[derive(FromPrimitive)]
enum Command {
    QueryValidPaths = 1,
    QueryPathInfos = 2,
    // cmdDumpStorePath = 3,
    ImportPaths = 4,
    // cmdExportPaths = 5,
    // cmdBuildPaths = 6,
    // cmdQueryClosure = 7,
    // cmdBuildDerivation = 8,
    // cmdAddToStoreNar = 9,
}

pub trait Store {
    // TODO(akavel): try to make it accept both &["foo"] and vec![String::from("foo")]. See however:
    // - https://stackoverflow.com/q/54225766
    // and optionally:
    // - https://github.com/rust-lang/rust/issues/22031
    // - https://stackoverflow.com/a/41180422
    // - https://stackoverflow.com/q/48734211
    // TODO(akavel): return an Iterator<String>
    fn query_valid_paths(&mut self, paths: &mut dyn Iterator<Item = &str>) -> Vec<String>;
}

struct NopHandler {}

impl nar::Handler for NopHandler {
    fn create_directory(&mut self, _path: &str) {}
    fn create_file(
        &mut self,
        _path: &str,
        _executable: bool,
        _size: u64,
        _contents: &mut impl Read,
    ) {
    }
    fn create_symlink(&mut self, _path: &str, _target: &str) {}
}
