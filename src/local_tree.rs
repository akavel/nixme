use crate::err::Result;
use crate::nar;
use failure::{self, Fail};
use std::os::unix::fs::PermissionsExt;
use std::{fs, io, path::{Path, PathBuf}};

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

    fn create_file(
        &mut self,
        path: &str,
        executable: bool,
        size: u64,
        contents: &mut impl io::Read,
    ) -> Result<()> {
        let mut f = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(self.rooted(path)?)?;
        f.set_len(size)?;
        if executable {
            f.set_permissions(fs::Permissions::from_mode(0o777))?;
        }
        let n = io::copy(contents, &mut f)?;
        if n != size {
            raise!(LocalTreeError::BadFileContentsLength {
                expected: size,
                actual: n
            });
        }
        Ok(())
    }

    fn create_symlink(&mut self, path: &str, target: &str) -> Result<()> {
        std::os::unix::fs::symlink(target, self.rooted(path)?)?;
        Ok(())
    }
}

impl LocalTree {
    pub fn new(root: &Path) -> Self {
        Self{
            root: root.to_path_buf(),
        }
    }

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
        if cleaned == PathBuf::new() {
            Ok(self.root.clone())
        } else {
            Ok(self.root.join(cleaned))
        }
    }
}

#[derive(Debug, Fail)]
enum LocalTreeError {
    #[fail(
        display = "bad file contents length, expected {}, got {}",
        expected, actual
    )]
    BadFileContentsLength { expected: u64, actual: u64 },
    #[fail(display = "too many '..' segments")]
    TooManyDotDot,
}

#[cfg(test)]
mod tests {
    use std::io::Read;
    use std::path::{Path, PathBuf};
    use crate::nar::Handler;
    use super::LocalTree;

    #[test]
    fn simple_tree() {
        let tmp_dir = tempname();
        let mut tree = LocalTree::new(&tmp_dir);
        tree.create_directory("").unwrap();
        tree.create_directory("/foo").unwrap();
        tree.create_directory("/foo/bar").unwrap();
        tree.create_directory("/foo/bar/../../boo").unwrap(); // = "/boo"
        let mut buf = r#"lasjdöaxnasd
asdom 12398
ä"§Æẞ¢«»”alsd
zażółć gęślą jaźń
"#.as_bytes();
        tree.create_file("/foo/data", false, buf.len() as u64, &mut buf).unwrap();
        let mut buf = "echo hello world".as_bytes();
        tree.create_file("/foo/script.sh", true, buf.len() as u64, &mut buf).unwrap();
        tree.create_symlink("/run", "foo/script.sh").unwrap();
    }

    #[test]
    fn simple_file() {
        let tmp = tempname();
        let mut tree = LocalTree::new(&tmp);
        let mut buf = "dummy".as_bytes();
        tree.create_file("", false, buf.len() as u64, &mut buf).unwrap();
    }

    #[test]
    fn bad_mkdir_order() {
        let tmp_dir = tempname();
        let mut tree = LocalTree::new(&tmp_dir);
        tree.create_directory("").unwrap();
        tree.create_directory("/fee").unwrap();
        let result = tree.create_directory("/foo/fum");
        assert!(result.is_err());
    }

    fn tempname() -> PathBuf {
        use rand::{thread_rng, Rng};
        use rand::distributions::Alphanumeric;
        let random: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();
        PathBuf::from(std::env::temp_dir()).join("nixme-".to_string() + &random)
    }
}
