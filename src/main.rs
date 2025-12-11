use std::process::Command;

use clap::Parser;
use emcc::compiler::Compiler;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    path: String,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    lex: bool,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    parse: bool,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    codegen: bool,
}

fn main() {
    let args = Args::parse();

    if args.lex {
        println!("lex");
    }

    let file_name: &str = args.path.split('.').collect::<Vec<_>>()[0];

    let preprocessed_name = file_name.to_string() + ".i";

    let asm_name = file_name.to_string() + ".s";

    let output_name = file_name.to_string() + ".out";

    println!("{}", file_name);

    let _preprocessed_file = Command::new("gcc")
        .args(vec!["-E", "-P", &args.path, "-o", &preprocessed_name])
        .output()
        .unwrap();

    // Call compiler with preprocessed file
    let _compile = Command::new("gcc")
        .args(vec!["-S", &preprocessed_name, "-o", &asm_name])
        .output()
        .unwrap();

    let _call_linker = Command::new("gcc")
        .args(vec![&asm_name, "-o", &output_name])
        .output()
        .unwrap();
}
