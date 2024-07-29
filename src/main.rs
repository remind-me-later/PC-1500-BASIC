#[forbid(unsafe_code)]
mod ast;
mod cfg;
mod tac;
mod tokens;

// TODO: use clap for argument parsing
fn main() {
    // Read file from first argument
    let input = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    let mut tokens = tokens::Lexer::new(&input);

    while let Some(token) = tokens.next_token() {
        print!("{} ", token);
    }
    println!();

    // match ast::parse(&input) {
    //     Ok((_, program)) => {
    //         let printer = ast::Printer::new();
    //         let output = printer.build(&program);
    //         println!("Ast:\n{}", output);
    //         let type_checker = ast::SemanticChecker::new(&program);
    //         type_checker.check().unwrap();
    //         let (tac, const_data) = tac::Builder::new(&program).build();

    //         println!("data:\n{:?}\n", const_data);
    //         println!("start:\n{}", tac);

    //         let mut cfg = cfg::Builder::new(tac).build();
    //         println!("Original CFG:\n{}", cfg);
    //         cfg.constant_fold();
    //         println!("Constant folded cfg:\n{}", cfg);
    //     }
    //     Err(err) => eprintln!("Error parsing program: {:?}", err),
    // }
}
