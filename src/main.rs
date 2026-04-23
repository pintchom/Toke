use clap::Parser;
use toke::cli::{Args, Commands};

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Build {
            file,
            hex,
            output,
            verbose,
        } => {
            let source = match std::fs::read_to_string(&file) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error reading '{}': {}", file.display(), e);
                    std::process::exit(2);
                }
            };

            let mut lexer = toke::lexer::Lexer::new(&source);
            let tokens = match lexer.tokenize() {
                Ok(t) => {
                    if verbose {
                        println!("Lexed {} tokens", t.len());
                    }
                    t
                }
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            };

            let mut parser = toke::parser::Parser::new(tokens, &source);
            let contract = match parser.parse() {
                Ok(c) => {
                    if verbose {
                        println!("Parsed contract: {}", c.name);
                    }
                    c
                }
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            };

            let result = toke::analyzer::analyze(&contract, &source);
            for warn in &result.warnings {
                eprintln!("{}", warn);
            }
            if !result.errors.is_empty() {
                for err in &result.errors {
                    eprintln!("{}", err);
                }
                std::process::exit(1);
            }
            if verbose {
                println!("Analysis: 0 errors, {} warnings", result.warnings.len());
            }

            let bytecode = match toke::codegen::generate(&contract) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            };

            if hex {
                println!("0x{}", ::hex::encode(&bytecode));
            } else {
                let out_path = output.unwrap_or_else(|| file.with_extension("bin"));
                if let Err(e) = std::fs::write(&out_path, &bytecode) {
                    eprintln!("Error writing output: {}", e);
                    std::process::exit(2);
                }
                println!("Output: {}", out_path.display());
            }
        }

        Commands::Lint { file } => {
            let source = match std::fs::read_to_string(&file) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error reading '{}': {}", file.display(), e);
                    std::process::exit(2);
                }
            };

            let mut lexer = toke::lexer::Lexer::new(&source);
            let tokens = match lexer.tokenize() {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            };

            let mut parser = toke::parser::Parser::new(tokens, &source);
            let contract = match parser.parse() {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            };

            let result = toke::analyzer::analyze(&contract, &source);
            if result.errors.is_empty() && result.warnings.is_empty() {
                println!("✓ No errors found, no warnings");
            } else {
                for err in &result.errors {
                    eprintln!("{}", err);
                }
                for warn in &result.warnings {
                    eprintln!("{}", warn);
                }
                if !result.errors.is_empty() {
                    std::process::exit(1);
                }
            }
        }

        Commands::Init => {
            println!("toke init wizard not yet implemented.");
        }
    }
}
