extern crate clap;

use std::fs;

use clap::{App, Arg};

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
            Arg::with_name("input-file-name")
                .help("Input file")
                .takes_value(true)
                .index(2)
                .required(true),
        )
        .arg(
            Arg::with_name("output-file-name")
                .help("Output file")
                .takes_value(true)
                .index(3)
                // TODO: Let the user decide if they simply wants to print to stdout or to a file.
                .required(true),
        )
        // Optional arguments
        .arg(
            Arg::with_name("output-variable-name")
                .short("variable-name")
                .help("Specify the name of the output variable.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output-length")
                .help("If specified, the length of the vector will also be generated.")
                .takes_value(true),
        )
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
    let input_file_name = matches.value_of("input-file-name").unwrap();
    let output_file_name = matches.value_of("output-file-name").unwrap();
    let output_variable_name = matches.value_of("output-variable-name").unwrap_or("stdin");
    let always_escape = matches.is_present("always-escape");

    println!(
        "language:{} input_file:{} output_file:{} always_escape:{} output_variable_name:{}",
        language, input_file_name, output_file_name, always_escape, output_variable_name
    );

    let r = '\r' as i32;
    let n = '\n' as i32;
    let t = '\t' as i32;
    let q = '\"' as i32;
    let s = '\\' as i32;
    // TODO: Is it possible to integrate rayon?
    let contents = fs::read(input_file_name)
        // TODO: Figure out a way to optionally perform this...
        .expect("Something went wrong reading the file.")
        .iter()
        .map(|x| {
            let c = *x as i32;
            if always_escape {
                format!(
                    "\\{}{}{}",
                    ('0' as u8 + ((c & 0o700) >> 6) as u8) as char,
                    ('0' as u8 + ((c & 0o070) >> 3) as u8) as char,
                    ('0' as u8 + ((c & 0o007) >> 0) as u8) as char
                )
            } else if c >= 32 && c <= 126 && c != '"' as i32 && c != '\\' as i32 && c != '?' as i32 && c != ':' as i32 && c != '%' as i32 {
                format!("{}", c as u8 as char)
            } else if c == r {
                format!("\\r")
            } else if c == n {
                format!("\\n")
            } else if c == t {
                format!("\\t")
            } else if c == q {
                format!("\"")
            } else if c == s {
                format!("\\")
            } else {
                format!(
                    "\\{}{}{}",
                    ('0' as u8 + ((c & 0o700) >> 6) as u8) as char,
                    ('0' as u8 + ((c & 0o070) >> 3) as u8) as char,
                    ('0' as u8 + ((c & 0o007) >> 0) as u8) as char
                )
            }
        })
        .collect::<Vec<String>>()
        .iter()
        .map(|x| x.chars())
        .flatten()
        .collect::<String>();

    match language {
        "cpp" | "c++" => {
            // TODO: Figure out how to create the name...
            println!("const unsigned char {}[] = \"{}\";", output_variable_name, contents);
            println!("const int {}_len =\"{}\";", output_variable_name, contents.len());
        }
        _ => panic!(format!("Unknown language:{}. If you think this tool should support this language extension, please submit a PR.", language))
    }
}
