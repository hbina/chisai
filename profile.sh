cargo build --release
ls -l target/debug/chisai
hyperfine --warmup 3 "xxd -i target/debug/chisai"
hyperfine --warmup 3 "xxd-rs generate target/debug/chisai"
hyperfine --warmup 3 "./strliteral target/debug/chisai"
hyperfine --warmup 3 "target/release/chisai --format=hex target/debug/chisai cpp"