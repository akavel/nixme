extern crate mock_io;
use io_dump::Packets;
use mock_io::Builder;
use std::collections::HashSet;
use std::io::Read;
use std::io::Write;

use nixme;
use nixme::Store;

#[test]
fn handshake_then_eof() {
    let transcript = br#"
->  0s  16 bytes
eb 9d 0c 39 00 00 00 00 04 02 00 00 00 00 00 00   ...9............ |

<-  0s  16 bytes
cb ee 52 54 00 00 00 00 04 02 00 00 00 00 00 00   ..RT............ |
"#;
    let mut mock = Builder::from_packets(Packets::new(&transcript[..])).build();
    let mut store = TestStore::new();
    nixme::serve(&mut store, &mut mock).unwrap();

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
fn query_valid_paths_with_one_missing_mock() {
    let transcript = br#"
// 8138  read(0, "\353\235\f9\0\0\0\0\4\2\0\0\0\0\0\0", 32768) = 16
->  0s  16 bytes
eb 9d 0c 39 00 00 00 00 04 02 00 00 00 00 00 00   ...9............ |

// 8138  write(1, "\313\356RT\0\0\0\0\4\2\0\0\0\0\0\0", 16) = 16
<-  0s  16 bytes
cb ee 52 54 00 00 00 00 04 02 00 00 00 00 00 00   ..RT............ |

// 8138  read(0, "\1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\2\0\0\0\0\0\0\0"..., 32768) = 160
->  0s  160 bytes
01 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00   ................ |
00 00 00 00 00 00 00 00 02 00 00 00 00 00 00 00   ................ |
36 00 00 00 00 00 00 00 2f 6e 69 78 2f 73 74 6f   6......./nix/sto |
72 65 2f 67 32 79 6b 35 34 68 69 66 71 6c 73 6a   re/g2yk54hifqlsj |
69 68 61 33 73 7a 72 34 71 33 63 63 6d 64 7a 79   iha3szr4q3ccmdzy |
72 64 76 2d 67 6c 69 62 63 2d 32 2e 32 37 00 00   rdv-glibc-2.27.. |
36 00 00 00 00 00 00 00 2f 6e 69 78 2f 73 74 6f   6......./nix/sto |
72 65 2f 6e 6b 70 34 63 6b 35 73 63 79 67 6b 6a   re/nkp4ck5scygkj |
6b 38 37 6e 72 36 77 36 31 67 62 32 33 6c 69 38   k87nr6w61gb23li8 |
32 39 6d 2d 68 65 6c 6c 6f 2d 32 2e 31 30 00 00   29m-hello-2.10.. |

// 8138  write(1, "\1\0\0\0\0\0\0\0006\0\0\0\0\0\0\0/nix/store/g2yk5"..., 72) = 72
<-  0s  72 bytes
01 00 00 00 00 00 00 00 36 00 00 00 00 00 00 00   ........6....... |
2f 6e 69 78 2f 73 74 6f 72 65 2f 67 32 79 6b 35   /nix/store/g2yk5 |
34 68 69 66 71 6c 73 6a 69 68 61 33 73 7a 72 34   4hifqlsjiha3szr4 |
71 33 63 63 6d 64 7a 79 72 64 76 2d 67 6c 69 62   q3ccmdzyrdv-glib |
63 2d 32 2e 32 37 00 00                           c-2.27..         |
"#;
    let mut mock = Builder::from_packets(Packets::new(&transcript[..])).build();
    let mut store = TestStore {
        has_paths: ["/nix/store/g2yk54hifqlsjiha3szr4q3ccmdzyrdv-glibc-2.27"]
            .iter()
            .map(|x| x.to_string())
            .collect(),
    };
    nixme::serve(&mut store, &mut mock).unwrap();
}

#[test]
fn existing_pkg_iodump() {
    let mut mock = Builder::open("tests/transcripts/a01-existing-pkg.iodump")
        .unwrap()
        .build();
    let mut store = TestStore::new();
    nixme::serve(&mut store, &mut mock).unwrap();

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
    let mut store = TestStore::new();
    nixme::serve(&mut store, &mut mock).unwrap();

    // Mock is empty.
    let mut buf = [0; 16];
    assert_eq!(
        0,
        mock.read(&mut buf).unwrap(),
        "iodump not yet exhausted for reading"
    );
    assert!(mock.write(b"X").is_err()); // TODO(akavel): something better should be used here
}

struct TestStore {
    has_paths: HashSet<String>,
}

impl TestStore {
    fn new() -> Self {
        TestStore {
            has_paths: HashSet::new(),
        }
    }
}

impl Store for TestStore {
    fn query_valid_paths(&mut self, paths: &mut dyn Iterator<Item = &str>) -> Vec<String> {
        let mut result = Vec::new();
        for p in paths {
            println!("{:#?}", &p);
            if self.has_paths.contains(p) {
                result.push(p.to_string());
            }
        }
        result
    }
}
