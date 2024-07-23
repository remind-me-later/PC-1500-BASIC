mod ast_printer;
mod dag;
mod parser;
mod semantic_check;
mod symbol_table;

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
        Ok((_, program)) => {
            let printer = ast_printer::AstPrintVisitor::new();
            let output = printer.build(&program);
            println!("Ast:\n{}", output);
            println!("Debug AST:\n{:#?}", program);
            let symbol_table = symbol_table::SymbolTableBuilderVisitor::new(&program).build();
            println!("Symbols:\n{}", symbol_table);
            let type_checker = semantic_check::SemanticCheckVisitor::new(&symbol_table, &program);
            let res = type_checker.check();
            println!("Type errors: {:?}", res);
        }
        Err(err) => eprintln!("Error parsing program: {:?}", err),
    }
}
