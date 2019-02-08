{.experimental: "codeReordering".}
import unittest
import streams
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
