#!/bin/sh
cargo run

measure_file() {
    env LANC='C' wc -c
}
test_gzip() {
    rawSize=$(measure_file <$1)
    compressedSize4=$(gzip -4 <$1 | measure_file)
    compressedSize9=$(gzip -9 <$1 | measure_file)
    echo "$1"
    echo "raw:     $rawSize bytes"
    echo "gzip -4: $compressedSize4 bytes"
    echo "gzip -9: $compressedSize9 bytes"
    echo ''
}

echo '7-bit predictable looking numbers'
echo 'encoded as an ASCII string:'
test_gzip output.ascii

echo 'packed into bytes'
test_gzip output.packed

echo 'encoded as an ASCII string, 1 byte per line:'
test_gzip output.ascii_8

echo 'encoded as an ASCII string, 72 bytes per line:'
test_gzip output.ascii_576