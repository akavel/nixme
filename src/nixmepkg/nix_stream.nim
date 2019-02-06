# Helpers for serializing/deserializing basic data types to/from Nix communication protocol
# format/stream.
# Based on:
#  - https://gist.github.com/jbeda/5c79d2b1434f0018d693 (copied from Figure 5.2 in Eelco's thesis
#    at http://nixos.org/~eelco/pubs/phd-thesis.pdf)
#  - https://github.com/NixOS/nix/blob/7b0b349085cf7cddb61e49b809d2be7ac28fe53e/src/libutil/serialise.cc

{.experimental: "codeReordering".}
import streams
import strutils

type
  NixStream = ref object
    base: Stream

proc wrap(stream: Stream): NixStream =
  return NixStream(base: stream)

using
  s: NixStream

proc flush(s) = s.base.flush()

proc write*(s; v: uint64) =
  let buf = [
    chr(v shr 56 and 0xff),
    chr(v shr 48 and 0xff),
    chr(v shr 40 and 0xff),
    chr(v shr 32 and 0xff),
    chr(v shr 24 and 0xff),
    chr(v shr 16 and 0xff),
    chr(v shr 8 and 0xff),
    chr(v and 0xff)]
  s.base.write(buf)

proc write*(s; v: string) =
  s.write(uint64(v.len))
  # FIXME(akavel): SUPER IMPORTANT: what encoding is used by Nix for strings in the protocol?
  s.base.write(v)
  s.base.write(' '.repeat(pad(uint64(v.len))))

proc write*(s; v: bool) =
  s.write(if v: 1 else: 0)

proc write*(s; v: openArray[string]) =
  s.write(uint64(v.len))
  for str in v:
    s.write(str)

## TODO: wrapper thanks to folks on Nim chat:
#proc read*[T:uint64|bool](s): T =
#  when T is uint64:
#    ...
#  elif T is bool:
#    ...

proc read_uint64*(s): uint64 =
  let buf = s.base.readStr(8)
  return buf[0].uint64 * 0x01000000_00000000'u64 +
         buf[1].uint64 * 0x00010000_00000000'u64 +
         buf[2].uint64 * 0x00000100_00000000'u64 +
         buf[3].uint64 * 0x00000001_00000000'u64 +
         buf[4].uint64 * 0x00000000_01000000'u64 +
         buf[5].uint64 * 0x00000000_00010000'u64 +
         buf[6].uint64 * 0x00000000_00000100'u64 +
         buf[7].uint64 * 0x00000000_00000001'u64

proc read_blob*(s): (uint64, Stream) =
  let n = s.read_uint64()
  let blob = new(Blob)
  blob.parent = s
  blob.n = n
  blob.padding = pad(n).uint8
  blob.readDataImpl = blobReadData
  blob.atEndImpl = blobAtEnd
  blob.closeImpl = blobClose
  return (n, blob)

proc expect*(s; want: uint64) =
  let have = s.read_uint64()
  if have != want:
    raise "expected $# (hex $#), got $# (hex $#)" % [want, toHex(want), have, toHex(have)]

proc expect*(s; want: string) =
  let have = s.read_str_ascii(want.len)
  if have != want:
    raise "expected '$#', got '$#'" % [want, have]

type
  Blob = ref object of Stream
    parent: NixStream
    n: uint64
    padding: uint8

proc blobReadData(s: Stream, buffer: pointer, bufLen: int): int =
  let blob = Blob(s)
  let n = if blob.n < bufLen.uint64: blob.n.int else: bufLen
  result = blob.parent.base.readData(buffer, n)
  blob.n -= result.uint64

proc blobAtEnd(s: Stream): bool =
  return Blob(s).n <= 0

proc blobClose(s: Stream) =
  let blob = Blob(s)
  if blob.n > 0'u64:
    # TODO(akavel): optimize to avoid allocating a string
    discard s.readAll()
  if blob.padding > 0'u8:
    # TODO(akavel): is it safe to discard the result, or do we have to verify length?
    discard blob.parent.base.readStr(blob.padding.int)
    blob.padding = 0

# Internal function, used to calculate length of 0-padding for byte slices in Nix protocol.
# n=1 => pad=7;  n=2 => pad=6;  n=7 => pad=1;  n=8 => pad=0
func pad(n: uint64): uint64 =
    (8'u64 - n mod 8) mod 8
