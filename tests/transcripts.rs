use nixme;
extern crate mock_io;
use mock_io::Builder;
use std::io::Read;
use std::io::Write;

#[test]
fn existing_pkg_iodump() {
    let mut mock = Builder::open("tests/transcripts/a01-existing-pkg.iodump")
        .unwrap()
        .build();
    nixme::serve(&mut mock).unwrap();

    // Mock is empty.
    let mut buf = [0; 16];
    assert_eq!(
        0,
        mock.read(&mut buf).unwrap(),
        "iodump not yet exhausted for reading"
    );
    assert!(mock.write(b"X").is_err()); // TODO(akavel): something better should be used here
}
