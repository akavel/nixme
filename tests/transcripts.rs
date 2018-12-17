use nixme;
extern crate mock_io;
use mock_io::Builder;
use io_dump::Packets;
use std::io::Read;
use std::io::Write;

#[test]
fn handshake_then_eof() {
    let transcript = br#"
->  0s  16 bytes
eb 9d 0c 39 00 00 00 00 04 02 00 00 00 00 00 00   ...9............ |

<-  0s  16 bytes
cb ee 52 54 00 00 00 00 04 02 00 00 00 00 00 00   ..RT............ |
"#;
    let mut mock = Builder::from_packets(Packets::new(&transcript[..])).build();
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

#[test]
fn failed_pkg_receive_iodump() {
    let mut mock = Builder::open("tests/transcripts/b01-failed-import.iodump")
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

