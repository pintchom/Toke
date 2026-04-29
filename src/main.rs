use clap::Parser;
use dialoguer::{Confirm, Input};
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
            println!("\nWelcome to Toke\n");

            let name: String = Input::new()
                .with_prompt("Contract name")
                .validate_with(|input: &String| {
                    if input.is_empty() {
                        return Err("Contract name cannot be empty");
                    }
                    if !input
                        .chars()
                        .next()
                        .map(|c| c.is_alphabetic())
                        .unwrap_or(false)
                    {
                        return Err("Must start with a letter");
                    }
                    if !input.chars().all(|c| c.is_alphanumeric() || c == '_') {
                        return Err("Only letters, numbers, and underscores allowed");
                    }
                    Ok(())
                })
                .interact_text()
                .unwrap();

            let default_symbol = name.chars().take(3).collect::<String>().to_uppercase();
            let symbol: String = Input::new()
                .with_prompt(format!("Token symbol (default: {})", default_symbol))
                .default(default_symbol)
                .interact_text()
                .unwrap();

            let decimals: u64 = Input::new()
                .with_prompt("Decimals (default: 18)")
                .default(18u64)
                .validate_with(|input: &u64| {
                    // mirrors check_decimals_range in analyzer.rs
                    if *input <= 77 {
                        Ok(())
                    } else {
                        Err("Decimals must be between 0 and 77")
                    }
                })
                .interact_text()
                .unwrap();

            let supply: u64 = Input::new()
                .with_prompt("Total supply")
                .validate_with(|input: &u64| {
                    // mirrors check_supply in analyzer.rs
                    if *input > 0 {
                        Ok(())
                    } else {
                        Err("Supply must be greater than 0")
                    }
                })
                .interact_text()
                .unwrap();

            let mintable = Confirm::new()
                .with_prompt("Mintable?")
                .default(false)
                .interact()
                .unwrap();

            let capped: Option<u64> = if mintable {
                let want_cap = Confirm::new()
                    .with_prompt("Add a supply cap?")
                    .default(false)
                    .interact()
                    .unwrap();
                if want_cap {
                    Some(
                        Input::new()
                            .with_prompt("Max supply cap")
                            .validate_with(|input: &u64| {
                                // mirrors check_capped_gte_supply in analyzer.rs
                                if *input >= supply {
                                    Ok(())
                                } else {
                                    Err("Cap must be greater than or equal to supply")
                                }
                            })
                            .interact_text()
                            .unwrap(),
                    )
                } else {
                    None
                }
            } else {
                None
            };

            let burnable = Confirm::new()
                .with_prompt("Burnable?")
                .default(false)
                .interact()
                .unwrap();

            // Build the .tc source
            let mut out = format!("contract {} {{\n", name);
            out += &format!("    symbol \"{}\"\n", symbol);
            out += &format!("    decimals {}\n", decimals);
            out += &format!("    supply {}\n", supply);
            if mintable {
                out += "    mintable\n";
            }
            if burnable {
                out += "    burnable\n";
            }
            if let Some(cap) = capped {
                out += &format!("    capped {}\n", cap);
            }
            out += "}\n";

            let filename = format!("{}.tc", name.to_lowercase());
            std::fs::write(&filename, &out).expect("failed to write .tc file");

            println!("\nGenerated {}\n", filename);
            println!("{}", out);
            println!("Run 'toke build {}' to compile.", filename);
        }
    }
}
