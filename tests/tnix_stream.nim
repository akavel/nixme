import unittest
import streams
import nixmepkg/nix_stream

suite "basic read primitives":
  test "read_uint64 is little endian":
    let
      input = newStringStream("\x01\x02\x03\x04\x05\x06\x07\x08")
      nix = wrap_nix_stream(input)
      output = nix.read_uint64()
    check output == 0x08070605_04030201'u64

  test "read_uint64 with sign bit set":
    let
      input = newStringStream("\xf1\xf2\xf3\xf4\xf5\xf6\xf7\xff")
      nix = wrap_nix_stream(input)
      output = nix.read_uint64()
    check output == 0xfff7f6f5_f4f3f2f1'u64

  test "read_uint64 twice in a sequence":
    let
      input = newStringStream("\x01\x02\x03\x04\x05\x06\x07\x08" &
        "\x11\x12\x13\x14\x15\x16\x17\x18")
      nix = wrap_nix_stream(input)
    check nix.read_uint64() == 0x08070605_04030201'u64
    check nix.read_uint64() == 0x18171615_14131211'u64

  test "read_str_ascii internal padding":
    let
      input = newStringStream("\x03\x00\x00\x00\x00\x00\x00\x00" &
        "abc45678" &
        "\xbe\xba\xba\xab\xed\xfe\xef\xbe")
      nix = wrap_nix_stream(input)
    check nix.read_str_ascii(20) == "abc"
    # verify that a subsequent value reads correctly
    check nix.read_uint64() == 0xbeeffeed_abbababe'u64

  test "read_str_ascii empty string":
    let
      input = newStringStream("\x00\x00\x00\x00\x00\x00\x00\x00" &
        "\xbe\xba\xba\xab\xed\xfe\xef\xbe")
      nix = wrap_nix_stream(input)
    check nix.read_str_ascii(20) == ""
    # verify that a subsequent value reads correctly
    check nix.read_uint64() == 0xbeeffeed_abbababe'u64

  test "read_str_ascii with no padding required":
    let
      input = newStringStream("\x08\x00\x00\x00\x00\x00\x00\x00" &
        "abcdefgh" &
        "\xbe\xba\xba\xab\xed\xfe\xef\xbe")
      nix = wrap_nix_stream(input)
    check nix.read_str_ascii(20) == "abcdefgh"
    # verify that a subsequent value reads correctly
    check nix.read_uint64() == 0xbeeffeed_abbababe'u64

