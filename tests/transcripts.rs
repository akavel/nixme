// use nixme;
extern crate mock_io;
use mock_io::Builder;
use std::io::Read;

#[test]
fn existing_pkg_iodump() {
    let mut mock = Builder::open("tests/transcripts/a01-existing-pkg.iodump").unwrap().build();
    let mut buf = [0; 11];
    assert_eq!(11, mock.read(&mut buf).unwrap());
    assert_eq!(&buf[..11], b"hello world");
}
