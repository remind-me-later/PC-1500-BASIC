#[forbid(unsafe_code)]
mod ast;
mod cfg;
mod tac;

use std::fs;

use clap::{Arg, Command};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pass {
    Lex,
    Parse,
    Sem,
    Tac,
    Cfg,
}

impl clap::ValueEnum for Pass {
    fn value_variants<'a>() -> &'a [Self] {
        &[Pass::Lex, Pass::Parse, Pass::Sem, Pass::Tac, Pass::Cfg]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Pass::Lex => Some(clap::builder::PossibleValue::new("lex")),
            Pass::Parse => Some(clap::builder::PossibleValue::new("parse")),
            Pass::Sem => Some(clap::builder::PossibleValue::new("sem")),
            Pass::Tac => Some(clap::builder::PossibleValue::new("tac")),
            Pass::Cfg => Some(clap::builder::PossibleValue::new("cfg")),
        }
    }
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
                .value_parser(clap::builder::EnumValueParser::<Pass>::new())
                // TODO: change when the compiler is finished
                .default_value("parse")
                .required(false),
        )
        .get_matches();

    // Read file from first argument
    let input = fs::read_to_string(args.get_one::<String>("input").unwrap()).unwrap();

    let pass = *args.get_one::<Pass>("pass").unwrap();

    let tokens = ast::Lexer::new(&input);

    if pass == Pass::Lex {
        for token in tokens {
            println!("{}", token);
        }

        return;
    }

    let mut parser = ast::Parser::new(tokens);

    let (program, parse_errors) = parser.parse();

    if !parse_errors.is_empty() {
        println!("Errors parsing program:");
        for error in parse_errors {
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
        let sem_errors = sem_checker.check();

        match sem_errors {
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
            print!("string literals: ");
            for literal in literals {
                print!("{} ", literal);
            }
            println!();

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
