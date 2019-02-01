use nixme::{nar, stream::Stream, err::Result};
use std::fs::File;
use std::io::Read;

#[test]
fn parse_simple_nar() {
    let mut f = File::open("tests/nar-simple/simple.nar").unwrap();
    let mut stream = Stream::new(&mut f);
    let mut handler = MockHandler {
        buf: "\n".to_owned(),
    };
    nar::parse(&mut stream, &mut handler).unwrap();
    assert_eq!(
        handler.buf,
        r#"
dir 
dir /foo
file /foo/bar = 0

file /foo/baz = 0

file /foo/data = 77
lasjdöaxnasd
asdom 12398
ä"§Æẞ¢«»”alsd
zażółć gęślą jaźń

exec /foo/script.sh = 17
echo hello world

file /foo-x = 0

link /ln-dir -> foo

link /ln-file -> foo/script.sh

file /qux = 0

dir /zyx
"#
    );
}

struct MockHandler {
    pub buf: String,
}

impl nar::Handler for MockHandler {
    fn create_directory(&mut self, path: &str) -> Result<()> {
        // print!("dir {}\n", path);
        self.buf.push_str(&format!("dir {}\n", path));
        Ok(())
    }

    fn create_file(&mut self, path: &str, executable: bool, size: u64, contents: &mut impl Read) -> Result<()> {
        if executable {
            // print!("exec {} = {}\n", path, size);
            self.buf.push_str(&format!("exec {} = {}\n", path, size));
        } else {
            // print!("file {} = {}\n", path, size);
            self.buf.push_str(&format!("file {} = {}\n", path, size));
        }
        contents.read_to_string(&mut self.buf).unwrap();
        self.buf.push_str("\n");
        Ok(())
    }

    fn create_symlink(&mut self, path: &str, target: &str) -> Result<()> {
        // print!("link {} -> {}\n", path, target);
        self.buf.push_str(&format!("link {} -> {}\n\n", path, target));
        Ok(())
    }
}
