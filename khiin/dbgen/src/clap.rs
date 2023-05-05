use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub conversions_file: String,

    #[arg(short, long)]
    pub frequency_file: String,

    #[arg(short, long)]
    pub output_file: String,
}

impl Args {
    pub fn validate() -> Result<Self> {
        let args = Self::parse();

        let path = PathBuf::from(&args.conversions_file);
        if !path.exists() {
            return Err(anyhow!("Conversion file {:?} not found.", path));
        }

        let path = PathBuf::from(&args.frequency_file);
        if !path.exists() {
            return Err(anyhow!("Frequency file {:?} not found.", path));
        }

        Ok(args)
    }
}
