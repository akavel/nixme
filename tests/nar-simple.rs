use nixme::{nar, stream::Stream};
use std::fs::File;
use std::io::Read;

#[test]
fn parse_simple_nar() {
    let mut f = File::open("tests/nar-simple/simple.nar").unwrap();
    let mut stream = Stream { stream: &mut f };
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

file /qux = 0

dir /zyx
"#
    );
}

struct MockHandler {
    pub buf: String,
}

impl nar::Handler for MockHandler {
    fn create_directory(&mut self, path: &str) {
        // print!("dir {}\n", path);
        self.buf.push_str(&format!("dir {}\n", path));
    }

    fn create_file(&mut self, path: &str, executable: bool, size: u64, contents: &mut impl Read) {
        if executable {
            // print!("exec {} = {}\n", path, size);
            self.buf.push_str(&format!("exec {} = {}\n", path, size));
        } else {
            // print!("file {} = {}\n", path, size);
            self.buf.push_str(&format!("file {} = {}\n", path, size));
        }
        contents.read_to_string(&mut self.buf).unwrap();
        self.buf.push_str("\n");
    }

    fn create_symlink(&mut self, path: &str, target: &str) {
        // print!("link {} -> {}\n", path, target);
        self.buf.push_str(&format!("link {} -> {}\n", path, target));
    }
}
