use std::{fs, io, path::PathBuf};
use std::os::unix::fs::PermissionsExt;
use failure::{self, Fail};
use crate::err::Result;
use crate::nar;

pub struct LocalTree {
    pub root: PathBuf,
}

impl nar::Handler for LocalTree {
    // FIXME(akavel): add some protections in the following functions!

    // TODO(akavel): take a Path instead of &str
    fn create_directory(&mut self, path: &str) -> Result<()> {
        fs::create_dir(self.rooted(path)?)?;
        Ok(())
    }

    fn create_file(&mut self, path: &str, executable: bool, size: u64, contents: &mut impl io::Read) -> Result<()> {
        let mut f = fs::OpenOptions::new().write(true).create_new(true).open(self.rooted(path)?)?;
        f.set_len(size)?;
        if executable {
            f.set_permissions(fs::Permissions::from_mode(0o777))?;
        }
        let n = io::copy(contents, &mut f)?;
        if n != size {
            raise!(LocalTreeError::BadFileContentsLength { expected: size, actual: n });
        }
        Ok(())
    }

    fn create_symlink(&mut self, path: &str, target: &str) -> Result<()> {
        std::os::unix::fs::symlink(target, path)?;
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
    fn rooted(&mut self, path: &str) -> Result<PathBuf> {
        let mut cleaned = PathBuf::new();
        for segment in path.split("/") {
            match segment {
                "" | "." => continue,
                ".." => {
                    if !cleaned.pop() {
                        raise!(LocalTreeError::TooManyDotDot);
                    }
                }
                _ => cleaned.push(segment),
            }
        }
        Ok(self.root.join(cleaned))
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
