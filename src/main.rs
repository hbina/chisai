extern crate clap;
use clap::{App, Arg, SubCommand};

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
            Arg::with_name("always-escape").help(" Always escape every byte with an octal escape."),
        )
        .arg(Arg::with_name("line-length").short("l").help("test"))
        .get_matches();
}
