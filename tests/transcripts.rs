// use nixme;
extern crate mock_io;
use mock_io::Builder;

#[test]
fn existing_pkg_iodump() {
    let builder = Builder::open("tests/transcripts/a01-existing-pkg.iodump").unwrap();
}
