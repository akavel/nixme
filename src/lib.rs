use failure::{bail, Error, ResultExt};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::io::{ErrorKind, Read, Write};

use crate::stream::Stream;
pub mod nar;
pub mod stream;

// TODO: test 'serve()' for simplified scenario with just cmd 1, with 1 missing pkg, with testing store
// TODO: test 'serve()' for simplified scenario with just cmd 2

// Based on NIX/src/nix-store/nix-store.cc, opServe()
// Other references:
// - NIX/src/libstore/legacy-ssh-store.cc
pub fn serve(store: &'static mut Store, stream: &mut (impl Read + Write)) -> Result<(), Error> {
    let mut stream = Stream::new(stream);
    // Exchange initial greeting.
    let magic = stream
        .read_u64()
        .context("cannot read 'hello' magic number")?;
    if magic != SERVE_MAGIC_1 {
        bail!("protocol mismatch");
    }
    stream.write_u64(SERVE_MAGIC_2)?;
    stream.write_u64(SERVE_PROTOCOL_VERSION)?;
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
                println!("query v.p.!");
                let paths = stream.read_strings_ascii(100, 300)?;
                // let response = store.query_valid_paths(&mut paths.iter().map(|s|&**s));
                let response = store.query_valid_paths(&paths);
                stream.write_strings(&response);
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
    // cmdImportPaths = 4,
    // cmdExportPaths = 5,
    // cmdBuildPaths = 6,
    // cmdQueryClosure = 7,
    // cmdBuildDerivation = 8,
    // cmdAddToStoreNar = 9,
}

// fn handleQueryValidPaths(&mut stream: Stream) -> Result<(), Error> {
//     // TODO: read stuff
//     let _lock = stream.read_bool()?; // TODO[LATER]: implement `lock` handling
//     let _substitute = stream.read_bool()?; // TODO[LATER]: implement `substitute` handling
//     let _paths = stream.read_strings(100, 300)?;
//     // TODO: reply stuff
// }

pub trait Store {
    // TODO: try to make it accept both &["foo"] and vec![String::from("foo")]. See however:
    // - https://stackoverflow.com/q/54225766
    // and optionally:
    // - https://github.com/rust-lang/rust/issues/22031
    // - https://stackoverflow.com/a/41180422
    // - https://stackoverflow.com/q/48734211
    // fn query_valid_paths(&mut self, paths: impl IntoIterator<Item=AsRef<str>>) -> Vec<String>;
    fn query_valid_paths_inner(&mut self, paths: &mut dyn Iterator<Item=&str>) -> Vec<String>;
    // fn query_valid_paths<'a>(&mut self, paths: impl IntoIterator<Item = &'a str>) -> Vec<String>
    // where Self: Sized {
    //     let mut it = paths.into_iter();
    //     self.query_valid_paths_inner(&mut it)
    // }
}

// fn _ensure_trait_objectable(_: &dyn Store) { }

impl dyn Store {
    pub fn query_valid_paths<'a, S, I>(&'a mut self, paths: I) -> Vec<String>
    where
        I: IntoIterator<Item = &'a S>,
        S: AsRef<str> + 'a,
    {
        let mut it = paths.into_iter().map(|x| x.as_ref());
        self.query_valid_paths_inner(&mut it)
    }
    // pub fn query_valid_paths(&mut self, paths: impl IntoIterator<Item = impl AsRef<str>>) -> Vec<String> {
    // pub fn query_valid_paths<S>(&mut self, paths: impl IntoIterator<Item = S>) -> Vec<String> 
    // where S: AsRef<str> {
    //     let mut iter = paths.into_iter();
    //     // self.query_valid_paths_inner(&mut iter.map(|s|s.as_ref()))
    //     self.query_valid_paths_inner(&mut iter.map(AsRef::as_ref))
    //     // self.query_valid_paths_inner(iter.map(AsRef::as_ref))
    // }
}

// fn _test_qvp(s: &'static mut dyn Store) {
fn _test_qvp(s: impl Store) {
    let a = vec![String::from("foo")];
    let b = &["foo"];
    // s.query_valid_paths(&a);
    // s.query_valid_paths(b);
    s.query_valid_paths_inner(a.into_iter());
    s.query_valid_paths_inner(b.into_iter());
}
