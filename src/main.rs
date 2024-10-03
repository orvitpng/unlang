mod lexer;

use clap::Parser;
use std::{
    fs::File,
    io::{BufReader, IsTerminal, Write, stdout},
    process::exit,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    input: String,
    output: Option<String>,
}

fn main() {
    let cli = Args::parse();

    let input = match File::open(cli.input) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("error: unable to open input file: {}", e);
            exit(1);
        }
    };
    let reader = BufReader::new(input);
    let mut lexer = lexer::Lexer::new(reader);

    println!("{:#?}", lexer.next());
    println!("{:#?}", lexer.next());
    println!("{:#?}", lexer.next());

    // we dont care about output yet
    let _: Box<dyn Write> = match cli.output {
        Some(output) => match File::create(output) {
            Ok(file) => Box::new(file),
            Err(e) => {
                eprintln!("error: unable to create output file: {}", e);
                exit(1);
            }
        },
        None => {
            if stdout().is_terminal() {
                eprintln!("error: output cannot be printed to the terminal");
                exit(1);
            }
            Box::new(stdout())
        }
    };
}
