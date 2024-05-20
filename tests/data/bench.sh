echo "Benching genson-rs"

echo "[genson-rs] Running benchmarks for 50MB file"
time ./target/release/genson-rs ./tests/data/test_small_50mb.json -d newline
time ./target/release/genson-rs ./tests/data/test_small_50mb.json -d newline
time ./target/release/genson-rs ./tests/data/test_small_50mb.json -d newline

echo "[genson-rs] Running benchmarks for 500MB file"
time ./target/release/genson-rs ./tests/data/test_medium_500mb.json -d newline
time ./target/release/genson-rs ./tests/data/test_medium_500mb.json -d newline
time ./target/release/genson-rs ./tests/data/test_medium_500mb.json -d newline

echo "[genson-rs] Running benchmarks for 1GB file"
time ./target/release/genson-rs ./tests/data/test_large_1gb.json -d newline
time ./target/release/genson-rs ./tests/data/test_large_1gb.json -d newline
time ./target/release/genson-rs ./tests/data/test_large_1gb.json -d newline

echo "[genson-rs] Running benchmarks for 3GB file"
time ./target/release/genson-rs ./tests/data/test_large_3gb.json -d newline
time ./target/release/genson-rs ./tests/data/test_large_3gb.json -d newline
time ./target/release/genson-rs ./tests/data/test_large_3gb.json -d newline

echo "[genson-rs] Running benchmarks for 3GB large JSON array file"
time ./target/release/genson-rs ./tests/data/test_large_3gb_full_json_array.json
time ./target/release/genson-rs ./tests/data/test_large_3gb_full_json_array.json
time ./target/release/genson-rs ./tests/data/test_large_3gb_full_json_array.json

echo "Benching GenSON (Python)"

echo "[GenSON] Running benchmarks for 50MB file"
time genson ./tests/data/test_small_50mb.json
time genson ./tests/data/test_small_50mb.json
time genson ./tests/data/test_small_50mb.json

echo "[GenSON] Running benchmarks for 500MB file"
time genson ./tests/data/test_medium_500mb.json
time genson ./tests/data/test_medium_500mb.json
time genson ./tests/data/test_medium_500mb.json

echo "[GenSON] Running benchmarks for 1GB file"
time genson ./tests/data/test_large_1gb.json
time genson ./tests/data/test_large_1gb.json
time genson ./tests/data/test_large_1gb.json

echo "[GenSON] Running benchmarks for 3GB large JSON array file"
time genson ./tests/data/test_large_3gb_full_json_array.json
time genson ./tests/data/test_large_3gb_full_json_array.json
time genson ./tests/data/test_large_3gb_full_json_array.json