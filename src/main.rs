mod ast;
mod parser;

use parser::Parser;
use typed_arena::Arena;

// TODO: use clap for argument parsing
fn main() {
    // Read file from first argument
    let input = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    let stmt_arena = Arena::new();
    let expr_arena = Arena::new();
    let str_arena = Arena::new();

    let parser = Parser::new(&stmt_arena, &expr_arena, &str_arena);

    match parser.parse(&input) {
        Ok((_, ast)) => println!("{}", ast),
        Err(err) => eprintln!("Error parsing program: {:?}", err),
    }
}
