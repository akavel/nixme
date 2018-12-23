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
file foo/bar
"
    );
}

struct MockHandler {
    pub buf: String,
}

impl nar::Handler for MockHandler {
    fn create_directory(&mut self, path: &str) {
        self.buf.push_str(&format!("dir {}\n", path));
    }

    fn create_file(&mut self, path: &str, executable: bool, size: u64, contents: &mut impl Read) {
        if executable {
            self.buf.push_str(&format!("exec {} = {}\n", path, size));
        } else {
            self.buf.push_str(&format!("file {} = {}\n", path, size));
        }
    }

    fn create_symlink(&mut self, path: &str, target: &str) {
        self.buf.push_str(&format!("link {} -> {}\n", path, target));
    }
}
