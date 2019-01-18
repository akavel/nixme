use failure::Error;
use std::collections::HashSet;
use std::env;
use std::io;

use nixme;

fn main() -> std::result::Result<(), Error> {
    // println!("Hello, world!");
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut stdio = ReadWrite {
        read: &mut stdin.lock(),
        write: &mut stdout.lock(),
    };
    let mut store = LocalStore {
        paths: env::args().collect(),
    };
    nixme::serve(&mut store, &mut stdio)
}

struct ReadWrite<'a, R, W>
where
    R: io::Read,
    W: io::Write,
{
    read: &'a mut R,
    write: &'a mut W,
}

impl<'a, R, W> io::Read for ReadWrite<'a, R, W>
where
    R: io::Read,
    W: io::Write,
{
    fn read(&mut self, buf: &mut [u8]) -> std::result::Result<usize, io::Error> {
        self.read.read(buf)
    }
}

impl<'a, R, W> io::Write for ReadWrite<'a, R, W>
where
    R: io::Read,
    W: io::Write,
{
    fn write(&mut self, buf: &[u8]) -> std::result::Result<usize, io::Error> {
        self.write.write(buf)
    }
    fn flush(&mut self) -> std::result::Result<(), io::Error> {
        self.write.flush()
    }
}

struct LocalStore {
    paths: HashSet<String>,
}

impl LocalStore {
    fn new() -> Self {
        LocalStore {
            paths: HashSet::new(),
        }
    }
}

impl nixme::Store for LocalStore {
    fn query_valid_paths(&mut self, paths: &mut dyn Iterator<Item = &str>) -> Vec<String> {
        let mut result = Vec::new();
        for p in paths {
            // println!("{:#?}", &p);
            if self.paths.contains(p) {
                result.push(p.to_string());
            }
        }
        result
    }
}
