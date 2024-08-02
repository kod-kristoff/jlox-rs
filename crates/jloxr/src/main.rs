use std::{
    env,
    fs::read_to_string,
    io::{self, Write},
    path::Path,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        eprintln!("usage: jloxr [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file<P: AsRef<Path>>(path: P) {
    let src = read_to_string(path).unwrap();
    run(src);
}

fn run_prompt() {
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Err(err) => todo!("handle error: {}", err),
            Ok(0) => break,
            Ok(_) => {}
        }
        run(line.trim_end().to_string());
    }
}

fn run(source: String) {
    println!("run '{}'", source)
}
