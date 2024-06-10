#!/bin/sh
cargo run
gzip -9 <output.ascii > output.ascii.gz
gzip -9 <output.packed > output.packed.gz
gzip -9 <output.ascii.txt > output.ascii.txt.gz

print_length() {
   	 stat -c '%n: %s bytes' "$1" "$1.gz"
}
echo '7-bit predictable looking numbers'
echo 'encoded as an ASCII string:'
print_length output.ascii
echo ''

echo 'packed into bytes'
print_length output.packed
echo ''

echo 'encoded as an ASCII string, 1 byte per line:'
print_length output.ascii.txt
echo ''