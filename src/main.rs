use std::{env, fs, process, process::Command};
use std::path::Path;
use colorex::Colorize;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("{} {}",
                  "Usage:".color("255,71,71"),
                  "wave <command> [arguments]");

        eprintln!("{}",
                  "Commands:".color("145,161,2"));

        eprintln!("  {}    {}",
                  "run <file>".color("38,139,235"),
                  "Execute the specified Wave file");

        eprintln!("  {}     {}",
                  "--version".color("38,139,235"),
                  "Show the CLI version");
        process::exit(1);
    }

    match args[1].as_str() {
        "--version" | "-V" => {
            println!("{}",
                     VERSION.color("2,161,47"));
            return;
        }
        "run" => unsafe {
            if args.len() < 3 {
                eprintln!("{} {}",
                          "Usage:".color("255,71,71"),
                          "wave run <file>");
                process::exit(1);
            }

            let file_path = &args[2];
            run_loa_file(file_path);
        }
        "help" => {
            println!("{}", "Options:".color("145,161,2"));
            println!("      {}       {}\n",
                     "run <file>".color("38,139,235"),
                     "Run the Wave code.");

            println!("{}", "Commands:".color("145,161,2"));
            println!("      {}    {}\n",
                     "-V, --version".color("38,139,235"),
                     "Verified the version of the Wave compiler.");
            return;
        }
        _ => {
            eprintln!("{} {}",
                      "Unknown command:".color("255,71,71"),
                      args[1]);
            eprintln!("{}",
                      "Use 'wave --version' or 'wave run <file>'".color("145,161,2"));
            process::exit(1);
        }
    }
}

unsafe fn run_loa_file(file_path: &str) {
    let code = fs::read_to_string(file_path).expect("Failed to read file");

    let mut lexer = Lexer::new(&code);
    let tokens = lexer.tokenize();

    let ast = parse(&tokens).expect("Failed to parse Wave code");

    println!("{}\n", code);
    println!("AST:\n{:#?}", ast);

    let mut interpreter = Interpreter::new();
    interpreter.execute(&ast);
}
