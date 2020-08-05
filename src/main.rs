extern crate clap;

use clap::{crate_version, App, Arg};
use rayon::prelude::*;
use std::fs;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let matches = App::new("chisai")
        .version(crate_version!())
        .author("Hanif Bin Ariffin <hanif.ariffin.4326@gmail.com>")
        .about("Transform binaries into embeddable code.")
        // Required arguments.
        .arg(
            Arg::with_name("language")
                .help("Desired language")
                .takes_value(true)
                .index(1)
                .required(true)
                .help("Desired language of the generated code."),
        )
        .arg(
            Arg::with_name("input-file-name")
                .help("Input file")
                .takes_value(true)
                .index(2)
                .required(true)
                .help("Input file"),
        )
        .arg(
            Arg::with_name("output-file-name")
                .help("Output file")
                .takes_value(true)
                .index(3),
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
                .help("WIP: Append every Nth character with a newline."),
        )
        .arg(Arg::with_name("ignore-whitespace").help("Ignore whitespaces."))
        .get_matches();

    let language = matches.value_of("language").unwrap();
    let input_file_name = matches.value_of("input-file-name").unwrap();
    let output_file_name = matches.value_of("output-file-name").unwrap();
    let output_variable_name = matches.value_of("output-variable-name").unwrap_or("stdin");
    let always_escape = matches.is_present("always-escape");
    let disable_const = matches.is_present("no-const");
    let output_file = matches.value_of("output-file-name");

    dbg!(
        "language:{} input_file:{} output_file:{} always_escape:{} output_variable_name:{} disable_const:{}",
        language, input_file_name, output_file_name, always_escape, output_variable_name,disable_const
    );

    let r = '\r' as i32;
    let n = '\n' as i32;
    let t = '\t' as i32;
    let q = '\"' as i32;
    let s = '\\' as i32;
    // TODO: Is it possible to integrate rayon?
    let content = fs::read(input_file_name)
        .expect("Something went wrong reading the file.")
        .par_iter()
        // TODO: It would be amazing if we could preallocate...
        .fold(
            || String::new(),
            |mut acc: String, x: &u8| {
                let c = *x as i32;

                if always_escape {
                    acc.push('\\' as u8 as char);
                    acc.push(('0' as u8 + ((c & 0o700) >> 6) as u8) as char);
                    acc.push(('0' as u8 + ((c & 0o070) >> 3) as u8) as char);
                    acc.push(('0' as u8 + ((c & 0o007) >> 0) as u8) as char);
                } else if c >= 32
                    && c <= 126
                    && c != '"' as i32
                    && c != '\\' as i32
                    && c != '?' as i32
                    && c != ':' as i32
                    && c != '%' as i32
                {
                    acc.push(c as u8 as char)
                } else if c == r {
                    acc.push('\r' as u8 as char);
                } else if c == n {
                    acc.push('\n' as u8 as char);
                } else if c == t {
                    acc.push('\t' as u8 as char);
                } else if c == q {
                    acc.push('\"' as u8 as char);
                } else if c == s {
                    acc.push('\\' as u8 as char);
                } else {
                    acc.push('\\' as u8 as char);
                    acc.push(('0' as u8 + ((c & 0o700) >> 6) as u8) as char);
                    acc.push(('0' as u8 + ((c & 0o070) >> 3) as u8) as char);
                    acc.push(('0' as u8 + ((c & 0o007) >> 0) as u8) as char);
                }
                acc
            },
        )
        .reduce(
            || String::new(),
            |mut a: String, b: String| {
                a.push_str(&b);
                a
            },
        );

    let output_string = match language {
        "c" | "cpp" | "c++" => {
            let constness = if disable_const {
                ""
            } else {
                "const "
            };
            format!("{}unsigned char {}[] = \"{}\";", constness, output_variable_name, content)
        }
        "python" | "py" => {
            format!("{}: str = {};", output_variable_name, content)
        }
        "java" => {
            format!("String {} = {};", output_variable_name, content)
        }
        _ => panic!(format!("Unknown language:{}. If you think this tool should support this language extension, please submit a PR.", language))
    };

    match output_file {
        Some(filename) => {
            let mut file = std::fs::File::create(filename)?;
            file.write_all(output_string.as_bytes())?;
            Ok(())
        }
        None => {
            println!("{}", &output_string);
            Ok(())
        }
    }
}
