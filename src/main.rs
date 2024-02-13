mod lox;
mod scanner;
mod token;

use crate::lox::Lox;
use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    #[allow(clippy::comparison_chain)]
    if args.len() > 2 {
        println!("Usage jlox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        Lox::new().run_file(&args[1])
    } else {
        Lox::new().run_prompt()
    }
}
