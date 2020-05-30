pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

pub struct EncodeArgs {
    pub file: PathBuf,
    pub chunk: String,
    pub message: String,
    pub out: Option<PathBuf>,
}

pub struct DecodeArgs {
    pub file: PathBuf,
    pub chunk: String,
}

pub struct RemoveArgs {
    pub file: PathBuf,
    pub chunk: String,
}

pub struct PrintArgs {
    pub file: PathBuf,
}
