mod ast;
mod cfg;
mod tac;

use ast::AstBuilder;
use ast::AstPrintVisitor;
use ast::SemanticCheckVisitor;
use ast::SymbolTableBuilderVisitor;
use tac::HirBuilder;

// TODO: use clap for argument parsing
fn main() {
    // Read file from first argument
    let input = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    let parser = AstBuilder::new();

    match parser.parse(&input) {
        Ok((_, program)) => {
            let printer = AstPrintVisitor::new();
            let output = printer.build(&program);
            println!("Ast:\n{}", output);
            let symbol_table = SymbolTableBuilderVisitor::new(&program).build();
            // println!("Symbols:\n{}", symbol_table);
            let type_checker = SemanticCheckVisitor::new(&symbol_table, &program);
            type_checker.check().unwrap();
            let (hir, const_data) = HirBuilder::new(&program).build();

            println!("data:\n{:?}\n", const_data);
            println!("start:\n{}", hir);

            let mut cfg = cfg::CFGBuilder::new(hir).build();
            // println!("Original cfg:\n{}", cfg);
            cfg.constant_fold();
            println!("Constant folded cfg:\n{}", cfg);
        }
        Err(err) => eprintln!("Error parsing program: {:?}", err),
    }
}
