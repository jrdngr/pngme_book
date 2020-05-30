use std::path::PathBuf;

use clap::Clap;

#[derive(Clap, Debug)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

// Write me!

#[cfg(test)]
mod assignment_tests {
    use super::*;
}
