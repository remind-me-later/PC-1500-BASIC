mod ast;
mod ssa;
mod tac;

// use ast::AstPrintVisitor;
use ast::AstBuilder;
use ast::SemanticCheckVisitor;
use ast::SymbolTableBuilderVisitor;
use tac::HirBuilder;

// TODO: use clap for argument parsing
fn main() {
    // Read file from first argument
    let input = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    let bump = bumpalo::Bump::new();

    let parser = AstBuilder::new(&bump);

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
            drop(bump);

            println!("data:\n{:?}\n", hir.1);
            println!("start:\n{}", hir.0);
        }
        Err(err) => eprintln!("Error parsing program: {:?}", err),
    }
}
