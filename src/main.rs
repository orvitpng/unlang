use std::{
    fs::File,
    io::{stdout, IsTerminal, Write},
};

use clap::Parser;
use eyre::{eyre, WrapErr};

pub fn main() -> eyre::Result<()> {
    let args = Args::parse();

    let input = File::open(&args.input)
        .wrap_err(format!("failed to open input file {}", args.input))?;
    let reader = std::io::BufReader::new(input);
    let _ = unlang::lexer::Lexer::new(reader);

    // we don't care about the output yet, but this is how we get it
    let _: Box<dyn Write> =
        match args.output {
            Some(output_path) => Box::new(File::open(&output_path).wrap_err(
                format!("failed to open output file {}", output_path),
            )?),
            None => {
                if stdout().is_terminal() {
                    Err(eyre!("output cannot be printed to the terminal"))?;
                }
                Box::new(stdout())
            }
        };

    Ok(())
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    input: String,
    output: Option<String>,
}
