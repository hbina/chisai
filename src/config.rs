use clap::ArgMatches;
use std::convert::TryInto;

pub enum Format {
    Binary,
    Octal,
    Hex,
}

impl std::convert::TryFrom<&str> for Format {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "binary" => Ok(Format::Binary),
            "octal" => Ok(Format::Octal),
            "hex" => Ok(Format::Hex),
            _ => Err(()),
        }
    }
}

pub enum Language {
    Cpp,
    Rust,
    Python,
    Java,
}

impl std::convert::TryFrom<&str> for Language {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "cpp" | "c++" => Ok(Language::Cpp),
            "python" | "py" => Ok(Language::Rust),
            "java" => Ok(Language::Python),
            "rust" | "rs" => Ok(Language::Java),
            _ => Err(()),
        }
    }
}

pub struct Config {
    language: Language,
    format: Format,
    prefix: String,
    separator: String,
    suffix: String,
    variable_per_line: usize,
}

impl Config {
    pub fn new(matches: ArgMatches) -> Result<Self, ()> {
        let language = matches.value_of("language").unwrap().try_into()?;
        // let input_file_name = matches.value_of("input-file-name").unwrap();
        // let output_file_name = matches.value_of("output-file-name");
        // let output_variable_name = matches.value_of("output-variable-name").unwrap_or("stdin");
        // let always_escape = matches.is_present("always-escape");
        // let disable_const = matches.is_present("no-const");
        let format = matches.value_of("format").unwrap_or("octal").try_into()?;
        let variable_per_line = matches
            .value_of("variable-per-line")
            .unwrap_or("10")
            .parse::<usize>()
            .expect("Unable to convert argument to variable-per-line to integer.");

        let (prefix, separator, suffix) = match language {
            Language::Cpp => ("{", ",", "};"),
            _ => return Err(()),
        };

        Ok(Config {
            language: language,
            format,
            prefix: prefix.to_owned(),
            separator: separator.to_owned(),
            suffix: suffix.to_owned(),
            variable_per_line,
        })
    }
}
