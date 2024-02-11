mod lox;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    #[allow(clippy::comparison_chain)]
    if args.len() > 2 {
        println!("Usage jlox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        lox::run_file(&args[1]);
    } else {
        lox::run_prompt();
    }
}
