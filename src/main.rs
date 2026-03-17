use clap::Parser;
use std::fs;
use std::process::{Command, Output};

mod cli;
mod compiler;

const PREPROCESSED_FILENAME: &'static str = "preprocessed_source.i";
const ASSEMBLY_FILENAME: &'static str = "assembly_source.s";

fn main() {
    let args: cli::Args = cli::Args::parse();
    let compilation_flags = args.compilation_flags;

    let _preprocess_command: Output = Command::new("gcc")
        .args([
            "-E",
            "-P",
            &args.input_filepath,
            "-o",
            PREPROCESSED_FILENAME,
        ])
        .output()
        .unwrap_or_else(|e| panic!("failed to run gcc preprocessing stage: {}", e));

    let preprocessed_file = fs::read_to_string(PREPROCESSED_FILENAME)
        .unwrap_or_else(|e| panic!("failed to open preprocessed source file: {}", e));

    // Compiling to assembly should happen here
    let compiler = compiler::Compiler::new(&compilation_flags);
    match compiler.compile(preprocessed_file) {
        Ok(_) => {} // Nothing for now but will be assembly down the line
        Err(e) => panic!("failed compilation: {}", e),
    }

    // Delete Preprocessing file
    if let Err(e) = fs::remove_file(PREPROCESSED_FILENAME) {
        println!(
            "failed to delete intermediate preprocessed source file: {}",
            e
        );
    }

    // Do not proceed with assembler and linker since
    // the compilation was not completed
    if compilation_flags.lex || compilation_flags.parse || compilation_flags.codegen {
        return;
    }

    let _assemble_command: Output = Command::new("gcc")
        .args([ASSEMBLY_FILENAME, "-o", &args.output_file])
        .output()
        .unwrap_or_else(|e| panic!("failed to run gcc assembling and linking stage: {}", e));

    // Delete Assembly file
    if let Err(e) = fs::remove_file(ASSEMBLY_FILENAME) {
        println!("failed to delete intermediate assembly source file: {}", e);
    }
}

// Currently assumes that all intermediate files will be created (and thus deleted) from the cwd.
