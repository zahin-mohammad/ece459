#!/bin/bash
cargo build --release

#echo "Testing SOLVE"
#for f in inputfiles/*.txt;
#do
##  echo "target/release/lab1 solve $(basename "$f")";
#  hyperfine -i "target/release/lab1 solve $(basename $f)";
#done

#echo "Testing VERIFY"
#for f in inputfiles/*.txt;
#do
#  hyperfine -i "target/release/lab1 verify $(basename $f)";
#done

#for i in {1..10}
#do
#  num_connections=$((i*10))
##  echo "${num_connections}";
#  hyperfine -i "target/release/lab1 verify 1.txt${num_connections}" "target/release/lab1 verify 10.txt ${num_connections}" "target/release/lab1 verify 100.txt ${num_connections}" "target/release/lab1 verify 200.txt ${num_connections}" "target/release/lab1 verify 300.txt ${num_connections}" "target/release/lab1 verify 400.txt ${num_connections}" "target/release/lab1 verify 500.txt ${num_connections}" --export-csv ${num_connections}.csv
#
#done

#for i in {1..10}
#do
#  num_connections=$((i*10))
#  hyperfine -i "target/release/lab1 verify 200.txt${num_connections}"  --export-csv ${num_connections}.csv
#done

#for i in {1..10}
#do
#  num_connections=$((i*10))
##  echo "${num_connections}";
#  hyperfine -i "target/release/lab1 verify 500.txt${num_connections}" "target/release/lab1 verify 10.txt ${num_connections}" "target/release/lab1 verify 100.txt ${num_connections}" "target/release/lab1 verify 200.txt ${num_connections}" "target/release/lab1 verify 300.txt ${num_connections}" "target/release/lab1 verify 400.txt ${num_connections}" "target/release/lab1 verify 500.txt ${num_connections}" --export-csv ${num_connections}.csv
#
#done
#hyperfine -i "target/release/lab1 verify 500.txt 10" "target/release/lab1 verify 500.txt 20" "target/release/lab1 verify 500.txt 30" "target/release/lab1 verify 500.txt 40" "target/release/lab1 verify 500.txt 50" "target/release/lab1 verify 500.txt 60" "target/release/lab1 verify 500.txt 70" "target/release/lab1 verify 500.txt 80" "target/release/lab1 verify 500.txt 90" "target/release/lab1 verify 500.txt 100" --export-csv data2.csv
hyperfine -i "target/release/lab1 verify 500.txt 3" "target/release/lab1 verify 500.txt 4" "target/release/lab1 verify 500.txt 16" "target/release/lab1 verify 500.txt 32" --export-csv data2.csv
