mod ast;
mod ast_printer;
mod parser;
mod semantic_analysis;
mod symbol_table;
mod type_checking;

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
        Ok((_, ast)) => {
            let printer = ast_printer::AstPrintVisitor::new();
            let output = printer.build(&ast);
            println!("Ast:\n{}", output);
            let symbol_table = symbol_table::SymbolTableBuilderVisitor::new().build(&ast);
            println!("Symbols:\n{}", symbol_table);
            let type_checker = type_checking::TypeCheckVisitor::new(&symbol_table);
            let res = type_checker.check(&ast);
            println!("Type errors: {:?}", res);
        }
        Err(err) => eprintln!("Error parsing program: {:?}", err),
    }
}
