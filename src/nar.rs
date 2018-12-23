use std::io::Read;

use failure;

use crate::stream::{error::ProtocolError, error::Result, Stream};

pub trait Handler {
    fn create_directory(&mut self, path: &str);
    fn create_file(&mut self, path: &str, executable: bool, size: u64, contents: &mut impl Read);
    fn create_symlink(&mut self, path: &str, target: &str);
}

pub fn parse(&mut stream: Stream, &mut handler: impl Handler) -> Result<()> {
    stream.expect_str(NAR_VERSION_MAGIC_1)?;
    parse_node(stream, "")
}

// FIXME(akavel): why `use crate::stream::protocol_error;` didn't work for me, even with
// #[macro_export]?
macro_rules! protocol_error {
    ($($arg:tt)*) => (
        Err(failure::Error::from(ProtocolError::Message {
            msg: format!($($arg)*)
        }))
    )
}

fn parse_node(&mut stream: Stream, &mut handler: impl Handler, path: String) -> Result<()> {
    stream.expect_str("(")?;
    stream.expect_str("type")?;
    match stream.read_str_ascii(20).as_ref() {
        "regular" => parse_file(&mut stream, &mut handler, path),
        "directory" => parse_directory(&mut stream, &mut handler, path),
        "symlink" => parse_symlink(&mut stream, &mut handler, path),
        other => {
            return protocol_error!(
                "unexpected node type, should be 'regular'/'directory'/'symlink': '{}'",
                other
            )
        }
    }
}

fn parse_file(&mut stream: Stream, &mut handler: impl Handler, path: String) -> Result<()> {
    let s = stream.read_str_ascii(20)?;
    let executable = s == "executable";
    let s = if executable {
        stream.expect_str("")?;
        stream.read_str_ascii(20)?
    } else {
        s
    };
    stream.expect_str("contents")?;
    let (size, &mut blob_stream) = stream.read_blob()?;
    handler.create_file(path, executable, size, &mut blob_stream)?;
    stream.expect(")")
}

fn parse_directory(&mut stream: Stream, &mut handler: impl Handler, path: String) -> Result<()> {
    handler.create_directory(path)?;
    let mut prev_name = "";
    loop {
        match stream.read_str_ascii(20).as_ref() {
            ")" => return Ok(()),
            s if s != "entry" => return protocol_error!("unexpected word in directory: '{}'", s),
        }
        stream.expect("(")?;
        stream.expect("name")?;
        let name = stream.read_str_ascii(max_name)?;
        if name == "" || name == "." || name == ".." || name.contains("/") || name.contains("\0") {
            return protocol_error!("node name contains invalid characters: '{}'", name);
        }
        if name <= prev_name {
            return protocol_error!("node name not sorted: '{}' <= '{}'", name, prev_name);
        }
        prev_name = name;
        stream.expect("node")?;
        parse_node(&mut stream, &mut handler, path + "/" + name)?;
    }
}

fn parse_symlink(&mut stream: Stream, &mut handler: impl Handler, path: String) -> Result<()> {
    handler.create_symlink(path, stream.read_str_ascii(max_target)?)?;
    stream.expect(")")
}

const max_name: u64 = 255; // FIXME(akavel): use some correct value here; MAX_PATH?
const max_target: u64 = 255; // FIXME(akavel): use some correct value here; MAX_PATH?

const NAR_VERSION_MAGIC_1: str = "nix-archive-1";
