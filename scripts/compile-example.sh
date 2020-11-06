# sh scripts/compile-example.sh expr.c
example="./examples/$1"
asm="./tmp/$1.s"
cargo run "$example" > "$asm" && clang "$asm"
