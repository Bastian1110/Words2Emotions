# This script if for compiling and running the normal asm excutable, 
#also copy the code repository to the main project repository

#Prepare for moving to another dic
echo "Preparing security copy ... 🐧"

rm -r target
rm pipeline.cbor
rm Cargo.lock

rm -r -f ../../JS/Svelte/word2emotions/text_emotions_rust/*

cp -r * ../../JS/Svelte/word2emotions/text_emotions_rust/

#Building normal ASM Rust project
echo "Building Rust Project ..."
cargo build --release

#Running build
echo "Running programm .. \n"
./target/release/text_emotions_rust

