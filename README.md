# chisai 

[![Crate](https://img.shields.io/crates/v/chisai.svg)](https://crates.io/crates/chisai)

Convert binaries into code that can be embedded statically.
Almost at full parity with [`strliteral`](https://github.com/mortie/strliteral) and definitely not against `xxd`.
Many features are WIP.

```
chisai 0.2.0
Hanif Bin Ariffin <hanif.ariffin.4326@gmail.com>
Transform binaries into embeddable code.

USAGE:
    chisai [FLAGS] [OPTIONS] <input-file-name> <language> [ARGS]

FLAGS:
        --always-escape    Always escape every byte with an octal escape.
    -h, --help             Prints help information
        --no-const         Generated variables are mutable.
    -V, --version          Prints version information

OPTIONS:
        --format <format>                          The format of the values.
        --variable-name <output-variable-name>     Specify the name of the output variable.
        --variable-per-line <variable-per-line>    For every N variable, append a newline.

ARGS:
    <input-file-name>     Input file.
    <language>            Desired language of the generated code.
    <output-file-name>    Output file.
    <output-length>       If specified, the length of the vector will also be generated.
```

## Benchmark

Using `hyperfine` by applying the programs to `target/debug/chisai` itself.

```shell
hbina@hbinalapt:~/git/chisai$ ./profile.sh 
    Finished release [optimized] target(s) in 0.02s
-rwxrwxr-x 2 hbina hbina 14234544 Aug  7 21:57 target/debug/chisai
Benchmark #1: xxd -i target/debug/chisai
  Time (mean ± σ):      1.465 s ±  0.019 s    [User: 1.459 s, System: 0.004 s]
  Range (min … max):    1.436 s …  1.499 s    10 runs
 
Benchmark #1: xxd-rs generate target/debug/chisai
  Time (mean ± σ):      5.630 s ±  0.231 s    [User: 2.358 s, System: 3.266 s]
  Range (min … max):    5.381 s …  6.057 s    10 runs
 
Benchmark #1: ./strliteral target/debug/chisai
  Time (mean ± σ):     124.2 ms ±   5.1 ms    [User: 120.1 ms, System: 3.9 ms]
  Range (min … max):   109.4 ms … 133.0 ms    23 runs
 
Benchmark #1: target/release/chisai --format=hex target/debug/chisai cpp
  Time (mean ± σ):     576.9 ms ±   7.1 ms    [User: 3.377 s, System: 0.285 s]
  Range (min … max):   565.4 ms … 588.1 ms    10 runs
 

```

`chisai` is ~3x faster than `xxd` and leagues faster than `xxd-rs`.
The program is still a lot slower than `strliteral`.
I think we could achieve performance parity if we can preallocate memory buffer for the output.
However, this is quite tricky to do without digging deep and writing necromonicons...
