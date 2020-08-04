trait Language {
    // NOTE: I am pretty sure String will create wrong values...
    fn create_string(&self) -> String;
}


struct Cpp {
    name: String,
    content: String,
    put_const: bool,
    put_length: bool,
    line_length: u32,
    // TODO: In the future we might want to enable formatter.
    // This option will likely supersede `line_length`.
    // formatter: Option<Formatter>,
}

impl Cpp {
    pub fn new(name: String, content: String) -> Cpp {
        Cpp {
            name,
            content,
            put_const: false,
            put_length: false,
            line_length: 100,
        }
    }

    pub fn add_variable_const(&mut self) -> &mut Cpp {
        self.put_const = true;
        self
    }

    pub fn add_variable_length(&mut self) -> &mut Cpp {
        self.put_length = true;
        self
    }

    pub fn set_line_length(&mut self, line: u32) -> &mut Cpp {
        self.line_length = line;
        self
    }
}

impl Language for Cpp {
    fn create_string(&self) -> String {
        let const_length = if self.put_const {
            "const ".len()
        } else {
            0
        };
        let put_variable_length = if self.put_length {
            format!("const int {}_len = {};", self.name
                    , self.content.len()).len()
        } else {
            0
        };
        let mut ouutput = String::with_capacity(self.content.len() + const_length + put_variable_length);

    }
}