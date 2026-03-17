use clap::Parser;

/// Minimal C compiler written in Rust
#[derive(Parser, Debug)]
#[command(
    name = "ycc",
    version, 
    about = "Yan's C compiler", 
    long_about = "You really shouldn't use my mediocre C compiler lol.")]
struct Args {
    /// Path to C source file to compile
    #[arg(short, long, required = true)]
    input_filepath: String,

    /// What to name the produced binary
    #[arg(short, long, default_value_t = String::from("forgortonamefile"))]
    name: String,

    /// Where the produced binary is to be stored
    #[arg(short, long, default_value_t = String::from("."))]
    output_filepath: String,
}

fn main() {
    let args = Args::parse();

    println!("Parsed arguments: {:?}", args);
}
