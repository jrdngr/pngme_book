use std::path::PathBuf;

pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

pub struct EncodeArgs {
    // Write me!
}

pub struct DecodeArgs {
    // Write me!
}

pub struct RemoveArgs {
    // Write me!
}

pub struct PrintArgs {
    // Write me!
}


// If you use this structure for your args, you can also use the following main function in main.rs
fn main() -> Result<()> {
    let args = todo!();

    match args {
        PngMeArgs::Encode(encode_args) => encode(encode_args),
        PngMeArgs::Decode(decode_args) => decode(decode_args),
        PngMeArgs::Remove(remove_args) => remove(remove_args),
        PngMeArgs::Print(print_args) => print_chunks(print_args),
    }
}
