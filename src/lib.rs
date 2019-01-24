use failure::{bail, Error, ResultExt};
use log::{debug, error};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::io::{ErrorKind, Read, Write};

#[macro_use]
pub mod err;
use crate::stream::Stream;
pub mod nar;
pub mod store;
pub mod stream;
// TODO(akavel): just re-publish LocalStore type from it
pub mod local_store;
pub mod local_tree;

// TODO: implement LocalStore with SQLite
// TODO: LATER: improve logging
// TODO: LATER: improve error handling to more helpful

// Based on NIX/src/nix-store/nix-store.cc, opServe()
// Other references:
// - NIX/src/libstore/legacy-ssh-store.cc
pub fn serve(store: &mut dyn store::Store, stream: &mut (impl Read + Write)) -> Result<(), Error> {
    let mut stream = Stream::new(stream);
    // Exchange initial greeting.
    stream.expect_u64(SERVE_MAGIC_1)?;
    debug!("got SERVE_MAGIC_1");
    stream.write_u64(SERVE_MAGIC_2)?;
    stream.write_u64(SERVE_PROTOCOL_VERSION)?;
    stream.flush()?;
    debug!("wrote SERVE_MAGIC_2");
    let _client_version = stream.read_u64().context("cannot read client version")?;
    debug!("read client_version");

    // Handle commands.
    loop {
        let cmd = match stream.read_u64() {
            Ok(x) => x,
            Err(e) => {
                debug!("failed to red cmd");
                // TODO(akavel): put below block in helper func, then call it from guard expression
                if let Some(ref cause) = e.downcast_ref::<std::io::Error>() {
                    if cause.kind() == ErrorKind::UnexpectedEof {
                        return Ok(());
                    }
                }
                return Err(e);
            }
        };
        debug!("read cmd: {}", cmd);
        match FromPrimitive::from_u64(cmd) {
            Some(Command::QueryValidPaths) => {
                debug!("cmd 1");
                let _lock = stream.read_bool()?; // TODO[LATER]: implement `lock` handling
                let _substitute = stream.read_bool()?; // TODO[LATER]: implement `substitute` handling
                let paths = stream.read_strings_ascii(100, 300)?;
                let response = store.query_valid_paths(&mut paths.iter().map(|s| &**s));
                stream.write_strings(&response)?;
                stream.flush()?;
            }
            Some(Command::QueryPathInfos) => {
                debug!("cmd 2");
                // FIXME(akavel): use some correct max length here
                let _paths = stream.read_strings_ascii(100, 300)?;
                // TODO(akavel): do we need to implement this, or is it ok to just fake it?
                stream.write_u64(0)?;
                stream.flush()?;
            }
            Some(Command::ImportPaths) => {
                debug!("cmd 4");
                loop {
                    let next = stream.read_u64()?;
                    if next == 0 {
                        break;
                    } else if next != 1 {
                        error!("cmd 4: got next={}", next);
                        bail!("input doesn't look like something created by 'nix-store --export'");
                    }
                    let mut handler = NopHandler {};
                    nar::parse(&mut stream, &mut handler)?;
                    stream.expect_u64(EXPORT_MAGIC)?;
                    // FIXME(akavel): use some correct max length here
                    const MAX_PATH: u64 = 255;
                    let _path = stream.read_str_ascii(MAX_PATH)?;
                    let _references = stream.read_strings_ascii(100, MAX_PATH)?;
                    let _deriver = stream.read_str_ascii(MAX_PATH)?;
                    // Ignore optional legacy signature.
                    if stream.read_u64()? == 1 {
                        let _ = stream.read_str_ascii(MAX_PATH)?;
                    }
                }
                stream.write_u64(1)?; // indicate success
                stream.flush()?;
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
const EXPORT_MAGIC: u64 = 0x4558_494e;

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

struct NopHandler {}

impl nar::Handler for NopHandler {
    fn create_directory(&mut self, _path: &str) -> std::result::Result<(), failure::Error> {
        Ok(())
    }
    fn create_file(
        &mut self,
        _path: &str,
        _executable: bool,
        _size: u64,
        _contents: &mut impl Read,
    ) -> std::result::Result<(), failure::Error> {
        Ok(())
    }
    fn create_symlink(&mut self, _path: &str, _target: &str) -> std::result::Result<(), failure::Error> {
        Ok(())
    }
}

// TODO: LATER: create temporary GC root
// TODO: LATER: if !repair && isValidPath(info.path) { return; }
// TODO: deletePath(realPath);
// TODO: restorePath(realPath, wrapperSource);
// TODO: LATER: verify hash
// TODO: LATER: verify size
// TODO: LATER: autoGC();
// TODO: LATER: canonicalisePathMetaData(realPath, -1);
// TODO: LATER: optimisePath(realPath);
// TODO: registerValidPath(info);
