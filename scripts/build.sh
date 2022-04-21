#!/bin/bash
SOURCE="./src/*"
BIN="./bin/"
for file in $SOURCE; do 
    file=$(realpath $file)
    rustc $file --out-dir $BIN
done