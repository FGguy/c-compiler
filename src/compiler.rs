use std::error;

use super::cli;

pub struct Compiler<'a> {
    _flags: &'a cli::CompilationFlags,
}

impl<'a> Compiler<'a> {
    pub fn new(flags: &'a cli::CompilationFlags) -> Compiler<'a> {
        Compiler { _flags: flags }
    }

    pub fn compile(&self, _preprocessed_file: String) -> Result<(), Box<dyn error::Error>> {
        unimplemented!();
    }
}
