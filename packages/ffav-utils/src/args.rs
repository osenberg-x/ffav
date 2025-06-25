use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// url of input file/address to process
    #[arg(short('i'), required(true))]
    pub url: String,

    /// Output file path 
    #[arg(short('o'), required(false), default_value(""))]
    pub output: String,
}

impl Args {
    pub fn new() -> Self {
        Args::parse()
    }
}