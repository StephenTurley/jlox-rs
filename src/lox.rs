use crate::scanner::Scanner;
use anyhow::Result;
use std::io::Write;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Lox {
        Lox { had_error: false }
    }
    pub fn run_file(&self, path: &str) -> Result<()> {
        let source = std::fs::read(path)?;
        self.run(source);

        if self.had_error {
            std::process::exit(65);
        }
        Ok(())
    }

    pub fn run_prompt(&mut self) -> Result<()> {
        loop {
            print!("> ");
            std::io::stdout().flush()?;

            let mut line = String::new();
            std::io::stdin().read_line(&mut line)?;

            self.run(line.into_bytes());
            self.had_error = false;
        }
    }

    fn run(&self, src: Vec<u8>) {
        let mut scanner = Scanner::new(src);
        let tokens = scanner.scan_tokens();
        tokens.iter().for_each(|tok| println!("{}", tok));
    }

    pub fn error(&mut self, line: u64, msg: &str) {
        self.report(line, "", msg);
    }

    fn report(&mut self, line: u64, loc: &str, msg: &str) {
        eprintln!("[line {}] Error{}: {}", line, loc, msg);
        self.had_error = true;
    }
}
