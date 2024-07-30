#[forbid(unsafe_code)]
mod ast;
mod cfg;
mod tac;

// TODO: use clap for argument parsing
fn main() {
    // Read file from first argument
    let input = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    let tokens = ast::Lexer::new(&input);

    println!("Tokens: ");
    for token in tokens {
        print!("{} ", token);
    }
    println!();

    let tokens = ast::Lexer::new(&input);

    let mut parser = ast::Parser::new(tokens);

    let program = parser.parse();

    {
        let printer = ast::Printer::new();
        let output = printer.build(&program);
        println!("Ast:\n{}", output);
        let sem_checker = ast::SemanticChecker::new(&program);
        sem_checker.check().unwrap();
        // let (tac, const_data) = tac::Builder::new(&program).build();

        // println!("data:\n{:?}\n", const_data);
        // println!("start:\n{}", tac);

        // let mut cfg = cfg::Builder::new(tac).build();
        // println!("Original CFG:\n{}", cfg);
        // cfg.constant_fold();
        // println!("Constant folded cfg:\n{}", cfg);
    }
}
