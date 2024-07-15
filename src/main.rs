mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let mut args = std::env::args();

    let input = std::fs::read_to_string(args.next_back().unwrap()).unwrap();

    let lexer = Lexer::new(input);

    let mut parser = Parser::new(lexer);
    parser.parse();

    for line in parser.get_lines() {
        println!("{} {}", line.0, line.1.borrow());
    }
}
