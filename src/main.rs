use std::{collections::HashMap, env::args, fs};

pub struct Compiler {
}

impl Compiler {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn compile(&mut self, raw_vasm : String) -> String {
        let tokens = raw_vasm.split_whitespace().collect::<Vec<&str>>();
        let mut label_table : HashMap<String, usize> = HashMap::new();

        let mut iter = tokens.iter().enumerate();
        while let Some((index, token)) = iter.next() {
            match token.to_owned() {
                "label" => {
                    let l = iter.next().expect("Compilation error - EOF after `label` keyword.");
                    label_table.insert(l.1.to_owned().to_owned(), index + 2);
                },
                _ => { continue; }
            }
        };

        let mut final_str = String::new();
        for (key, value) in &label_table {
            final_str.push_str(&format!("{} {}\n", key, value));
        };
        final_str.push_str("/// END COMPILER GENERATED LABEL TABLE ///\n\n");
        for token in &tokens {
            final_str.push_str(&format!("{} ", token));
        };
        final_str
    }
}

fn main() {
    let mut args = args().into_iter();

    let i = args.next();
    if let None = i {
        panic!("No input path provided!");
    }
    let input = i.unwrap();
    let o = args.next();
    if let None = o {
        panic!("No output path provided!");
    }
    let output = o.unwrap();

    let vasm = fs::read_to_string(input).expect("Input file not found or no permissions.");

    let mut compiler = Compiler::new();
    let vraw = compiler.compile(vasm);

    fs::write(output, vraw).expect("Failed to write output file!");
}