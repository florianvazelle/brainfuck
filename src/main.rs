use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write};

use brainfuck::{analyzer, compiler, lexer, parser};

fn main() {
    // Get command line parameters
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage : {} INPUT.bf OUTPUT.c", &args[0]);
        return;
    }

    // Open and read brainfuck file
    let file = File::open(&args[1]).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    // Tokenize
    let tokens = lexer::tokenize(&contents);

    // Parse
    let parsers = parser::parse(tokens.as_slice());

    let size = analyzer::analyze(parsers.as_slice());

    // Compile into C code
    let c_code = compiler::compile(parsers.as_slice(), size);

    // Write into a new C file
    let mut handle = File::create(&args[2]).unwrap();
    handle.write_all(c_code.as_bytes()).unwrap();
    handle.flush().unwrap();
}
