python3 generate.py
cargo run --release -- cuda input/cnn.csv input/in.csv output/out_cuda.csv
cargo run --release -- cpu input/cnn.csv input/in.csv output/out.csv
python3 compare.py