mod ast;
mod hir;

// use ast::AstPrintVisitor;
use ast::Parser;
use ast::SemanticCheckVisitor;
use ast::SymbolTableBuilderVisitor;
use hir::HirBuilder;
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
            // let printer = AstPrintVisitor::new();
            // let output = printer.build(&program);
            // println!("Ast:\n{}", output);
            let symbol_table = SymbolTableBuilderVisitor::new(&program).build();
            // println!("Symbols:\n{}", symbol_table);
            let type_checker = SemanticCheckVisitor::new(&symbol_table, &program);
            type_checker.check().unwrap();

            let hir = HirBuilder::new(&program).build();
            println!("data:\n{:?}\n", hir.1);
            println!("start:\n{}", hir.0);
        }
        Err(err) => eprintln!("Error parsing program: {:?}", err),
    }
}
