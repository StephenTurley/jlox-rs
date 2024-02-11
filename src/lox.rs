use std::io::Write;

pub fn run_file(path: &str) {
    let source = std::fs::read_to_string(path).unwrap();
    run(&source);
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
        println!("{}", line);
    }
}

fn run(_src: &str) {}
