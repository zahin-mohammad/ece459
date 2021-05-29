# Verify
cargo build --release
target/release/lab4 > out.txt
tail -5 out.txt > new_checksum.txt
rm -rf out.txt
diff original_checksum.txt new_checksum.txt

hyperfine --min-runs 1000 -i "target/release/lab4" > new_hyperfine.txt

# Flamegraph
make
mv flamegraph.svg new_flamegraph.svg
rm -rf *.data
rm -rf *.data.old
