mod tokenizer;
mod parser;

use std::io::{self, Write};
use tokenizer::Tokenizer;
use parser::{Parser, Query};

fn main() {
    loop{
        print!("sql>");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim().eq_ignore_ascii_case("exit") {
            break;
        }

        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize();
        println!("Tokens: {:?}", tokens);

        let mut parser = Parser::new(tokens);
        match parser.parse() {
            Ok(query) => println!("Parsed Query: {:?}", query),
            Err(e) => println!("Error {}", e),
        }
    }
}
