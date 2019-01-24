use std::io::Read;

use failure;

use crate::stream::{error::ProtocolError, Stream};
use crate::err::Result;

// Inflater? Unpacker? Materializer? Recipient? TreeBuilder? TreeWriter?
pub trait Handler {
    fn create_directory(&mut self, path: &str) -> Result<()>;
    fn create_file(
        &mut self,
        path: &str,
        executable: bool,
        size: u64,
        contents: &mut impl Read,
    ) -> Result<()>;
    fn create_symlink(
        &mut self,
        path: &str,
        target: &str,
    ) -> Result<()>;
}

pub fn parse<R, H>(stream: &mut Stream<R>, handler: &mut H) -> Result<()>
where
    R: Read,
    H: Handler,
{
    stream.expect_str(NAR_VERSION_MAGIC_1)?;
    parse_node(stream, handler, "")
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

fn parse_node<R, H>(stream: &mut Stream<R>, handler: &mut H, path: &str) -> Result<()>
where
    R: Read,
    H: Handler,
{
    stream.expect_str("(")?;
    stream.expect_str("type")?;
    match stream.read_str_ascii(20)?.as_ref() {
        "regular" => parse_file(stream, handler, path),
        "directory" => parse_directory(stream, handler, path),
        "symlink" => parse_symlink(stream, handler, path),
        other => {
            return protocol_error!(
                "unexpected node type, should be 'regular'/'directory'/'symlink': '{}'",
                other
            )
        }
    }
}

fn parse_file<R, H>(stream: &mut Stream<R>, handler: &mut H, path: &str) -> Result<()>
where
    R: Read,
    H: Handler,
{
    let s = stream.read_str_ascii(20)?;
    let executable = s == "executable";
    let s = if executable {
        stream.expect_str("")?;
        stream.read_str_ascii(20)?
    } else {
        s
    };
    if s != "contents" {
        return protocol_error!("unexpected word, should be 'contents': {}", s);
    }
    let (size, mut blob_stream) = stream.read_blob()?;
    handler.create_file(path, executable, size, &mut blob_stream)?;
    stream.expect_str(")")
}

fn parse_directory<R, H>(stream: &mut Stream<R>, handler: &mut H, path: &str) -> Result<()>
where
    R: Read,
    H: Handler,
{
    handler.create_directory(path)?;
    let mut prev_name = "".to_string();
    loop {
        match stream.read_str_ascii(20)?.as_ref() {
            ")" => return Ok(()),
            s if s != "entry" => return protocol_error!("unexpected word in directory: '{}'", s),
            _ => (),
        }
        stream.expect_str("(")?;
        stream.expect_str("name")?;
        let name = stream.read_str_ascii(MAX_NAME)?;
        if name == "" || name == "." || name == ".." || name.contains('/') || name.contains('\0') {
            return protocol_error!("node name contains invalid characters: '{}'", name);
        }
        if name <= prev_name {
            return protocol_error!("node name not sorted: '{}' <= '{}'", name, prev_name);
        }
        stream.expect_str("node")?;
        parse_node(stream, handler, &(path.to_owned() + "/" + &name))?;
        prev_name = name;
        stream.expect_str(")")?;
    }
}

fn parse_symlink<R, H>(stream: &mut Stream<R>, handler: &mut H, path: &str) -> Result<()>
where
    R: Read,
    H: Handler,
{
    stream.expect_str("target")?;
    handler.create_symlink(path, &stream.read_str_ascii(MAX_TARGET)?)?;
    stream.expect_str(")")
}

const MAX_NAME: u64 = 255; // FIXME(akavel): use some correct value here; MAX_PATH?
const MAX_TARGET: u64 = 255; // FIXME(akavel): use some correct value here; MAX_PATH?

const NAR_VERSION_MAGIC_1: &str = "nix-archive-1";
