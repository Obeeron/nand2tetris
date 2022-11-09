use clap::Parser;

mod symbol_table;
mod assembler;
mod error;

use assembler::Assembler;

#[derive(Parser)]
#[command(author = "Obeeron", version = "1.0")]
struct Cli {
    input_file: String,
    #[clap(short, long, help = "Compiled output file")]
    output: Option<String>,
    #[clap(short='F', default_value = "binary", help = "Format of compiled output")]
    output_format: OutputFormat,
    #[clap(short='i', long="save-pp-file", help = "Set this flag to save the intermediate preprocessed file")]
    save_preprocessed_file: bool,
}

#[derive(clap::ValueEnum, Clone)]
pub enum OutputFormat {
    Text,
    Binary,
}

fn main() {
    let cli = Cli::parse();
    if cli.input_file == "" {
        println!("No input file specified");
    }

    if let Some(output) = &cli.output {
        if cli.input_file == *output {
            println!("Input file and output file cannot be the same");
            return;
        }
    }

    // Instantiate the assembler
    let mut assembler = Assembler::new(cli.output_format, cli.save_preprocessed_file);
    match assembler.run(cli.input_file, cli.output) {
        Ok(_) => println!("Assembled successfully"),
        Err(e) => println!("{}", e),
    }
}