extern crate clap;

use std::fs;
use std::io::prelude::*;

use clap::{crate_version, App, Arg};
use rayon::prelude::*;

#[derive(Debug, Copy, Clone)]
enum Format {
    Binary,
    Octal,
    LowerHex,
    UpperHex,
}

impl From<&str> for Format {
    fn from(x: &str) -> Self {
        match x {
            "binary" => Format::Binary,
            "octal" => Format::Octal,
            "lowerhex" => Format::LowerHex,
            "upperhex" => Format::UpperHex,
            _ => Format::Octal,
        }
    }
}

fn main() -> std::io::Result<()> {
    let matches = App::new("chisai")
        .version(crate_version!())
        .author("Hanif Bin Ariffin <hanif.ariffin.4326@gmail.com>")
        .about("Transform binaries into embeddable code.")
        // Required arguments.
        .arg(
            Arg::with_name("input-file-name")
                .long("input-file-name")
                .help("Input file")
                .takes_value(true)
                .index(1)
                .required(true)
                .help("Input file"),
        )
        .arg(
            Arg::with_name("language")
                .long("language")
                .help("Desired language")
                .takes_value(true)
                .index(2)
                .required(true)
                .help("Desired language of the generated code."),
        )
        .arg(
            Arg::with_name("output-file-name")
                .long("output-file-name")
                .help("Output file")
                .takes_value(true)
                .index(3),
        )
        // Optional arguments
        .arg(
            Arg::with_name("output-variable-name")
                .long("variable-name")
                .help("Specify the name of the output variable.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output-length")
                .help("If specified, the length of the vector will also be generated.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("no-const")
                .long("no-const")
                .help("Generated variables are mutable."),
        )
        .arg(
            Arg::with_name("format")
                .long("format")
                .help("The format of the values.")
                .takes_value(true),
        )
        // NOTE: I don't quite get what this means...
        .arg(
            Arg::with_name("always-escape")
                .long("always-escape")
                .help("Always escape every byte with an octal escape."),
        )
        .arg(
            Arg::with_name("variable-per-line")
                .long("variable-per-line")
                .takes_value(true)
                .help("For every N variable, append a newline."),
        )
        .arg(
            Arg::with_name("ignore-whitespace")
                .long("ignore-whitespace")
                .help("Ignore whitespaces."),
        )
        .get_matches();

    let language = matches.value_of("language").unwrap();
    let input_file_name = matches.value_of("input-file-name").unwrap();
    let output_file_name = matches.value_of("output-file-name");
    let output_variable_name = matches.value_of("output-variable-name").unwrap_or("stdin");
    let always_escape = matches.is_present("always-escape");
    let disable_const = matches.is_present("no-const");
    let ignore_whitespace = matches.is_present("ignore-whitespace");
    let format = matches.value_of("format").unwrap_or("ii");

    dbg!(
        language,
        input_file_name,
        output_file_name,
        always_escape,
        output_variable_name,
        disable_const,
        ignore_whitespace,
        format
    );

    let content = if let Ok(x) = fs::read(input_file_name) {
        let mut content = x
            .par_iter()
            .fold(
                || String::new(),
                |mut s: String, c: &u8| {
                    match format.into() {
                        Format::Binary => {
                            s += &format!("{:#b}, ", c);
                        }
                        Format::Octal => {
                            s += &format!("0x{:#o}, ", c);
                        }
                        Format::LowerHex => {
                            s += &format!("{:#x}, ", c);
                        }
                        Format::UpperHex => {
                            s += &format!("{:#X}, ", c);
                        }
                    }
                    s
                },
            )
            .reduce(
                || String::new(),
                |mut a: String, b: String| {
                    a.push_str(&b);
                    a
                },
            );
        content.truncate(content.len() - 2);
        content
    } else {
        panic!("Something went wrong while reading the file.")
    };

    let output_string = match language {
        "c" | "cpp" | "c++" => {
            let constness = if disable_const {
                ""
            } else {
                "const "
            };
            format!("{}unsigned char {}[] = {{\n{}\n}};", constness, output_variable_name, content)
        }
        "python" | "py" => {
            format!("{}: str = {};", output_variable_name, content)
        }
        "java" => {
            format!("String {} = {};", output_variable_name, content)
        }
        _ => panic!(format!("Unknown language:{}. If you think this tool should support this language extension, please submit a PR.", language))
    };

    match output_file_name {
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
