extern crate clap;

use clap::{App, Arg, SubCommand};
use std::env;
use std::fs;

fn main() {
    let matches = App::new("chisai")
        .version("1.0")
        .author("Hanif Bin Ariffin <hanif.ariffin.4326@gmail.com>")
        .about("Transform binaries into embeddable code.")
        // Required arguments.
        .arg(
            Arg::with_name("language")
                .help("Desired language")
                .takes_value(true)
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("Input file")
                .takes_value(true)
                .index(2)
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .help("Output file")
                .takes_value(true)
                .index(3)
                // TODO: Let the user decide if they simply wants to print to stdout or to a file.
                .required(true),
        )
        // Optional arguments
        .arg(Arg::with_name("no-const").help("Generated variables are mutable."))
        // NOTE: I don't quite get what this means...
        .arg(
            Arg::with_name("always-escape")
                .long("always-escape")
                .help(" Always escape every byte with an octal escape."),
        )
        .arg(
            Arg::with_name("line-length")
                .long("line-length")
                .takes_value(true)
                .help("test"),
        )
        .arg(Arg::with_name("ignore-whitespace").help("Ignore whitespaces."))
        .get_matches();

    let language = matches.value_of("language").unwrap();
    let input_file = matches.value_of("input").unwrap();
    let output_file = matches.value_of("output").unwrap();
    let always_escape = matches.is_present("always-escape");

    println!(
        "language:{} input_file:{} output_file:{} always_escape:{}",
        language, input_file, output_file, always_escape
    );

    // TODO: Is it possible to integrate rayon?
    let contents = fs::read_to_string(input_file)
        // TODO: Figure out a way to optionally perform this...
        .expect("Something went wrong reading the file.");

    println!("const unsigned char stdin[] = {:#?};", contents);
    println!("const int stdin_len = {:#?};", contents.len());
}
