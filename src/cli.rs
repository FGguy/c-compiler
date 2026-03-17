use clap::Parser;

#[derive(Parser, Debug)]
pub struct CompilationFlags {
    /// run up to the lexer and stop before parsing
    #[arg(long, default_value_t = false)]
    pub lex: bool,

    /// run up to the parser and stop before assembly generation
    #[arg(long, default_value_t = false)]
    pub parse: bool,

    // run up to the codegen and stop before code emission
    #[arg(long, default_value_t = false)]
    pub codegen: bool,
}

/// Minimal C compiler written in Rust
#[derive(Parser, Debug)]
#[command(
    name = "ycc",
    version,
    about = "Yan's C compiler",
    long_about = "You really shouldn't use my mediocre C compiler lol."
)]
pub struct Args {
    /// Path to C source file to compile
    #[arg(short, long, required = true)]
    pub input_filepath: String,

    /// What to name the produced binary
    #[arg(short, long, default_value_t = String::from("forgortonamefile"))]
    pub output_file: String,

    #[command(flatten)]
    pub compilation_flags: CompilationFlags,
}
