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
            Arg::with_name("input-file-name")
                .long("input-file-name")
                .help("Input file")
                .takes_value(true)
                .index(1)
                .required(true)
                .help("Input file."),
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
                .help("Output file.")
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
        .get_matches();

    let language = matches.value_of("language").unwrap();
    let input_file_name = matches.value_of("input-file-name").unwrap();
    let output_file_name = matches.value_of("output-file-name");
    let output_variable_name = matches.value_of("output-variable-name").unwrap_or("stdin");
    let always_escape = matches.is_present("always-escape");
    let disable_const = matches.is_present("no-const");
    let format = matches.value_of("format").unwrap_or("octal");
    let variable_per_line = matches
        .value_of("variable-per-line")
        .unwrap_or("10")
        .parse::<usize>()
        .expect("Unable to convert argument to variable-per-line to integer.");

    dbg!(
        language,
        input_file_name,
        output_file_name,
        always_escape,
        output_variable_name,
        disable_const,
        format
    );

    let content = {
        let mut content = fs::read(input_file_name)?
            .par_iter()
            .enumerate()
            .fold(
                || String::new(),
                |mut s: String, (index, c): (usize, &u8)| {
                    let index = index + 1;
                    if (index % variable_per_line) == 1 {
                        s += "  ";
                    }
                    // TODO: This can be extracted out of this loop.
                    match format {
                        "binary" => {
                            s += &format!("0b{:0>8b}, ", c);
                        }
                        "octal" => {
                            s += &format!("0{:0>3o}, ", c);
                        }
                        "hex" => {
                            s += &format!("0x{:0>2X}, ", c);
                        }
                        "decimal" => {
                            s += &format!("{:0>3}, ", c);
                        }
                        _ => {
                            panic!("Invalid format {}", format);
                        }
                    }
                    if index != 0 && (index % variable_per_line) == 0 {
                        s.push('\n');
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
        // TODO: This is frankly quite arbitrary and ugly...
        // Consider fixing it...
        content.truncate(content.len() - 2);
        content
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
