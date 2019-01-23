use std::{fs, io, path::PathBuf};
use path_clean::{clean, PathClean};
use failure;

pub struct LocalTree {
    pub root: PathBuf,
}

impl nar::Handler for LocalTree {
    // FIXME(akavel): add some protections in the following functions!

    // TODO(akavel): take a Path instead of &str
    fn create_directory(&mut self, path: &str) -> std::result::Result<(), failure::Error> {
        fs::create_dir(self.rooted(path)?).err()?;
        Ok(())
    }

    fn create_file(&mut self, path: &str, executable: bool, size: u64, contents: &mut impl Read) -> std::result::Result<(), failure::Error> {
        let mut f = fs::OpenOptions::new().write(true).create_new(true).open(self.rooted(path)?)?;
        f.set_len(size)?;
        // f.set_permissions(
        let n = io::copy(contents, f)?;
        if n != size {
            return Err(BadFileContentsLength { expected: size, actual: n });
            // return format!("bad contents length, expected {}, got {}", size, n);
        }
        Ok(())
    }
}

impl LocalTree {
    /// Removes leading and trailing slashes and ".", and any repeated slashes, in a "/"-separated
    /// path. Simplifies any ".." by erasing preceding path segments, then prefixes path with
    /// self.root. Returns error in case there are any leading "..", or in case there are more ".."
    /// than preceding path segments.
    // FIXME(akavel): ensure we handle Win32 paths correctly
    // FIXME(akavel): ensure we correctly remove ".." prefixes
    // FIXME(akavel): https://github.com/rust-lang/rfcs/issues/2208#issuecomment-456840910
    fn rooted(&mut self, path: &str) -> std::result::Result<PathBuf, failure::Error> {
        let mut cleaned = PathBuf::new();
        for segment in path.split("/") {
            match segment {
                "", "." => continue,
                ".." => {
                    if !cleaned.pop() {
                        return Err(TooManyDotDot);
                        // return format!("too many '..' segments");
                    }
                }
                _ => cleaned.push(segment),
            }
        }
        self.root.join(cleaned)
    }
}

#[derive(Debug, Fail)]
enum LocalTreeError {
    #[fail(display = "bad file contents length, expected {}, got {}", expected, actual)]
    BadFileContentsLength {
        expected: u64,
        actual: u64,
    },
    #[fail(display = "too many '..' segments")]
    TooManyDotDot,
}
