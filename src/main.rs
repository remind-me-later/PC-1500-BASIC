#[forbid(unsafe_code)]
mod ast;
mod cfg;
mod tac;

use std::{fs::File, io::Read};

use clap::{builder::PossibleValuesParser, Arg, Command};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pass {
    Lex,
    Parse,
    Sem,
    Tac,
    Cfg,
}

// TODO: use clap for argument parsing
fn main() {
    let args = Command::new("sbc")
        .arg(
            Arg::new("input")
                .help("BASIC source file to compile")
                .value_name("FILE")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output file to write to")
                .required(false),
        )
        .arg(
            Arg::new("pass")
                .short('p')
                .long("pass")
                .value_name("PASS")
                .help("Compiler pass to run")
                .value_parser(PossibleValuesParser::new(&[
                    "lex", "parse", "sem", "tac", "cfg",
                ]))
                // TODO: change when the compiler is finished
                .default_value("parse")
                .required(false),
        )
        .get_matches();

    // Read file from first argument
    let input = {
        let mut file = File::open(args.get_one::<String>("input").unwrap()).unwrap();
        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();
        input
    };

    let pass = match args.get_one::<String>("pass").unwrap().as_str() {
        "lex" => Pass::Lex,
        "parse" => Pass::Parse,
        "sem" => Pass::Sem,
        "tac" => Pass::Tac,
        "cfg" => Pass::Cfg,
        _ => unreachable!(),
    };

    let tokens = ast::Lexer::new(&input);

    if pass == Pass::Lex {
        for token in tokens {
            println!("{:?}", token);
        }

        return;
    }

    let mut parser = ast::Parser::new(tokens);

    let (program, errors) = parser.parse();

    if !errors.is_empty() {
        println!("Errors parsing program:");
        for error in errors {
            println!("{}", error);
        }
    } else {
        if pass == Pass::Parse {
            let printer = ast::Printer::new();
            let output = printer.build(&program);
            println!("{}", output);
            return;
        }

        let sem_checker = ast::SemanticChecker::new(&program);
        let errors = sem_checker.check();

        match errors {
            Ok(_) => {
                if pass == Pass::Sem {
                    println!("No semantic errors found");
                    return;
                }
            }
            Err(errors) => {
                println!("Errors in semantic analysis:");
                for error in errors {
                    println!("{}", error);
                }
                return;
            }
        }

        let (tac, literals) = tac::Builder::new(&program).build();

        if pass == Pass::Tac {
            println!("string literals: {:?}\n", literals);
            println!("start:\n{}", tac);
            return;
        }

        if pass == Pass::Cfg {
            let cfg = cfg::Builder::new(tac).build();
            println!("{}", cfg);
            return;
        }
    }
}
