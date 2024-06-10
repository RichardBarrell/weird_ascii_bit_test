Needs gzip, sh and cargo.

Run `sh run_test.sh`.

The Rust program here builds a stream of repetitive bits with a repetitive
pattern that is *seven* bits long (not eight), and outputs the bits in a couple
of different formats. The shell script then checks how long they are, before
and after gzip.

Output on my machine looks like:

```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target\debug\weird_ascii_bit_test.exe`
Wrote packed file
Wrote ASCII file (1 line per byte)
Wrote ASCII file (1 line per 72 bytes)
Wrote ASCII file (oops all bytes)
7-bit predictable looking numbers
encoded as an ASCII string:
output.ascii
raw:     7168 bytes
gzip -4: 1281 bytes
gzip -9: 138 bytes

packed into bytes
output.packed
raw:     896 bytes
gzip -4: 515 bytes
gzip -9: 515 bytes

encoded as an ASCII string, 1 byte per line:
output.ascii_8
raw:     8064 bytes
gzip -4: 1749 bytes
gzip -9: 867 bytes

encoded as an ASCII string, 72 bytes per line:
output.ascii_576
raw:     7181 bytes
gzip -4: 1321 bytes
gzip -9: 178 bytes
```
