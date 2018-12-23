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
        "
dir 
dir /foo
file /foo/bar = 0
file /foo/baz = 0
file /foo/data = 77
exec /foo/script.sh = 17
file /foo-x = 0
file /qux = 0
dir /zyx
"
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
    }

    fn create_symlink(&mut self, path: &str, target: &str) {
        // print!("link {} -> {}\n", path, target);
        self.buf.push_str(&format!("link {} -> {}\n", path, target));
    }
}
