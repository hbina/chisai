# chisai 

Convert binaries into code that can be embedded statically.
Almost at full parity with [`strliteral`](https://github.com/mortie/strliteral) and definitely not against `xxd`.
Many features are WIP.

```
chisai 0.1.2
Hanif Bin Ariffin <hanif.ariffin.4326@gmail.com>
Transform binaries into embeddable code.

USAGE:
    chisai [FLAGS] [OPTIONS] <language> <input-file-name> [ARGS]

FLAGS:
        --always-escape         Always escape every byte with an octal escape.
    -h, --help                 Prints help information
        --ignore-whitespace    Ignore whitespaces.
        --no-const             Generated variables are mutable.
    -V, --version              Prints version information

OPTIONS:
        --line-length <line-length>               WIP: Append every Nth character with a newline.
        --variable-name <output-variable-name>    Specify the name of the output variable.

ARGS:
    <language>            Desired language of the generated code.
    <input-file-name>     Input file
    <output-file-name>    Output file
    <output-length>       If specified, the length of the vector will also be generated.

```

## Benchmark

Using `hyperfine` to benchmark by converting `[Doremi].Go!.Princess.Precure.28.[1280x720].[2F623257].mkv` renamed to `test.mkv` which is about `369.3MB`.

```shell
hbina085@hbinalapt:~/Downloads$ ls -l test.mkv 
-rwxrwxrwx 1 hbina085 hbina085 369327503 Jul 23 17:27 test.mkv
hbina085@hbinalapt:~/Downloads$ hyperfine --warmup 3 "chisai cpp test.mkv outfile" --export-json benchmark_chisai.json
Benchmark #1: chisai cpp test.mkv outfile
  Time (mean ± σ):      2.572 s ±  0.101 s    [User: 11.334 s, System: 2.397 s]
  Range (min … max):    2.415 s …  2.749 s    10 runs
 
hbina085@hbinalapt:~/Downloads$ hyperfine --warmup 3 "./strliteral test.mkv" --export-json benchmark_strliteral.json
Benchmark #1: ./strliteral test.mkv
  Time (mean ± σ):      3.971 s ±  0.077 s    [User: 3.854 s, System: 0.108 s]
  Range (min … max):    3.873 s …  4.095 s    10 runs
 

```

## strliteral

```json
{
  "results": [
    {
      "command": "./strliteral test.mkv",
      "mean": 4.0896332868,
      "stddev": 0.47063479103199835,
      "median": 3.9372541290000003,
      "user": 3.962159145,
      "system": 0.116312375,
      "min": 3.6839701245,
      "max": 5.1621161275,
      "times": [
        3.6839701245,
        3.6943990525,
        3.9264306605000003,
        3.7815495475,
        4.0116240505,
        3.9480775975,
        3.8907904335000003,
        4.1301359575,
        4.6672393165,
        5.1621161275
      ]
    }
  ]
}
```

## chisai

```json
{
  "results": [
    {
      "command": "chisai cpp test.mkv outfile",
      "mean": 2.57200334339,
      "stddev": 0.10101415479260571,
      "median": 2.55863462079,
      "user": 11.333895325,
      "system": 2.39657342,
      "min": 2.41521520579,
      "max": 2.74894131279,
      "times": [
        2.69010737279,
        2.52024189479,
        2.50440452979,
        2.5672032687899997,
        2.62540644779,
        2.62117424479,
        2.74894131279,
        2.47727318379,
        2.41521520579,
        2.55006597279
      ]
    }
  ]
}
```
