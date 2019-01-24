use dirs;
use log;
use simplelog;
use std::{fs, io};

use nixme::{self, local_store::LocalStore, err::Result};
// use nixme;

////////
//
// GOAL:
//
// Nixme is intended to be used as a Nix replacement for a `nix copy` target.
//
// MILESTONES
// - First of all, be able to create real files in a specified directory tree on disk,
//   unpacking them from received NAR data.
// - Then, correctly report NARs already received, to optimize data transferred from `nix copy`.
// - Eventually, support GCing - this will probably require storing references and GC roots. Might
//   want to implement some parts of the Nix SQLite database at this point.
// - LATER: implement full Nix SQLite database.

fn main() -> Result<()> {
    let mut log_file = dirs::data_local_dir().unwrap();
    log_file.push("nixme.log");
    simplelog::WriteLogger::init(
        simplelog::LevelFilter::Debug,
        simplelog::Config::default(),
        fs::File::create(log_file)?,
    )?;

    // println!("Hello, world!");
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut stdio = ReadWrite {
        read: &mut stdin.lock(),
        write: &mut stdout.lock(),
    };
    let mut store = LocalStore {
        // paths: env::args().collect(),
        paths: ["/nix/store/g2yk54hifqlsjiha3szr4q3ccmdzyrdv-glibc-2.27"]
            .iter()
            .map(|x| x.to_string())
            .collect(),
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
