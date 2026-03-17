use std::error;

use super::cli;

pub struct Compiler<'a> {
    _flags: &'a cli::CompilationFlags,
}

impl Compiler<'_> {
    pub fn new<'a>(flags: &'a cli::CompilationFlags) -> Compiler<'a> {
        Compiler { _flags: flags }
    }

    pub fn compile(&self, _preprocessed_file: String) -> Result<(), Box<dyn error::Error>> {
        unimplemented!();
    }
}
