{.experimental: "codeReordering".}
import unittest
import streams
import sets
import transcript
import nixmepkg/serve
import nixmepkg/local_store

suite "transcripts":
  test "handshake, then EOF":
    let session = transcript"""
-> # 0s  16 bytes
eb 9d 0c 39 00 00 00 00 04 02 00 00 00 00 00 00   # ...9............ |

<- # 0s  16 bytes
cb ee 52 54 00 00 00 00 04 02 00 00 00 00 00 00   # ..RT............ |
"""
    let store = LocalStore()
    store.serve(session, session)
    check session.atEnd

  test "query valid paths, with mocked one missing":
    let session = transcript"""
# 8138  read(0, "\353\235\f9\0\0\0\0\4\2\0\0\0\0\0\0", 32768) = 16
-> # 0s  16 bytes
eb 9d 0c 39 00 00 00 00 04 02 00 00 00 00 00 00   # ...9............ |

# 8138  write(1, "\313\356RT\0\0\0\0\4\2\0\0\0\0\0\0", 16) = 16
<- # 0s  16 bytes
cb ee 52 54 00 00 00 00 04 02 00 00 00 00 00 00   # ..RT............ |

# 8138  read(0, "\1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\2\0\0\0\0\0\0\0"..., 32768) = 160
-> # 0s  160 bytes
01 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00   # ................ |
00 00 00 00 00 00 00 00 02 00 00 00 00 00 00 00   # ................ |
36 00 00 00 00 00 00 00 2f 6e 69 78 2f 73 74 6f   # 6......./nix/sto |
72 65 2f 67 32 79 6b 35 34 68 69 66 71 6c 73 6a   # re/g2yk54hifqlsj |
69 68 61 33 73 7a 72 34 71 33 63 63 6d 64 7a 79   # iha3szr4q3ccmdzy |
72 64 76 2d 67 6c 69 62 63 2d 32 2e 32 37 00 00   # rdv-glibc-2.27.. |
36 00 00 00 00 00 00 00 2f 6e 69 78 2f 73 74 6f   # 6......./nix/sto |
72 65 2f 6e 6b 70 34 63 6b 35 73 63 79 67 6b 6a   # re/nkp4ck5scygkj |
6b 38 37 6e 72 36 77 36 31 67 62 32 33 6c 69 38   # k87nr6w61gb23li8 |
32 39 6d 2d 68 65 6c 6c 6f 2d 32 2e 31 30 00 00   # 29m-hello-2.10.. |

# 8138  write(1, "\1\0\0\0\0\0\0\0006\0\0\0\0\0\0\0/nix/store/g2yk5"..., 72) = 72
<- # 0s  72 bytes
01 00 00 00 00 00 00 00 36 00 00 00 00 00 00 00   # ........6....... |
2f 6e 69 78 2f 73 74 6f 72 65 2f 67 32 79 6b 35   # /nix/store/g2yk5 |
34 68 69 66 71 6c 73 6a 69 68 61 33 73 7a 72 34   # 4hifqlsjiha3szr4 |
71 33 63 63 6d 64 7a 79 72 64 76 2d 67 6c 69 62   # q3ccmdzyrdv-glib |
63 2d 32 2e 32 37 00 00                           # c-2.27..         |
"""
    let store = LocalStore(paths: ["/nix/store/g2yk54hifqlsjiha3szr4q3ccmdzyrdv-glibc-2.27"].toSet)
    store.serve(session, session)
    check session.atEnd

  test "existing-pkg.iodump":
    let session = transcript(openFileStream("tests/transcripts/a01-existing-pkg.iodump"))
    let store = LocalStore(paths: [
        "/nix/store/2kcrj1ksd2a14bm5sky182fv2xwfhfap-glibc-2.26-131",
        "/nix/store/aakgkcvw6j54zg38zrn1w00sgxx0zj8b-xz-5.2.3-bin",
        "/nix/store/chf54cl12ifswf6swh7kxpif477drihi-xz-5.2.3",
        ].toSet)
    store.serve(session, session)
    check session.atEnd

  test "pkg import iodump":
    let session = transcript(openFileStream("tests/transcripts/c00-import.iodump"))
    let store = LocalStore(paths: ["/nix/store/g2yk54hifqlsjiha3szr4q3ccmdzyrdv-glibc-2.27"].toSet)
    store.serve(session, session)
    check session.atEnd
