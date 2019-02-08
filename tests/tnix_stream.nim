import unittest
import streams
import nixmepkg/nix_stream

suite "basic read primitives":
  test "read_uint64 is big endian":
    let
      input = newStringStream("\x01\x02\x03\x04\x05\x06\x07\x08")
      nix = wrap_nix_stream(input)
      output = nix.read_uint64()
    check output == 0x01020304_05060708'u64

  test "read_uint64 with sign bit set":
    let
      input = newStringStream("\xff\xfe\xfd\xfc\xfb\xfa\xf9\xf1")
      nix = wrap_nix_stream(input)
      output = nix.read_uint64()
    check output == 0xfffefdfc_fbfaf9f1'u64

  test "two read_uint64 in sequence":
    let
      input = newStringStream("\x01\x02\x03\x04\x05\x06\x07\x08" &
        "\x11\x12\x13\x14\x15\x16\x17\x18")
      nix = wrap_nix_stream(input)
    check nix.read_uint64() == 0x01020304_05060708'u64
    check nix.read_uint64() == 0x11121314_15161718'u64

  test "read_str_ascii padding":
    let
      input = newStringStream("\x00\x00\x00\x00\x00\x00\x00\x03" &
        "abc45678" &
        "\xbe\xef\xfe\xed\xab\xba\xba\xbe")
      nix = wrap_nix_stream(input)
    check nix.read_str_ascii(20) == "abc"
    # verify that a subsequent value reads correctly
    check nix.read_uint64() == 0xbeeffeed_abbababe'u64

