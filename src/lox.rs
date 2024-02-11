use crate::scanner::Scanner;
use std::io::Write;

pub fn run_file(path: &str) {
    let source = std::fs::read_to_string(path).unwrap();
    run(source);
}

pub fn run_prompt() {
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut line = String::new();
        let result = std::io::stdin().read_line(&mut line);

        if result.is_err() {
            break;
        }
        run(line);
    }
}

fn run(src: String) {
    let scanner = Scanner::new(src);
    let tokens = scanner.scan_tokens();
    tokens.iter().for_each(|tok| println!("{}", tok));
}
