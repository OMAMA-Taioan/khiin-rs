mod clap;

use std::borrow::Cow;
use std::fs::read_to_string;

use anyhow::Result;
use simplelog::Config;
use simplelog::SimpleLogger;

use khiin::db::CsvFiles;
use khiin::db::Database;

use self::clap::Args;

pub fn main() -> Result<()> {
    SimpleLogger::init(log::LevelFilter::Debug, Config::default()).unwrap();
    log::debug!("Begin database generation");

    let result = match Args::validate() {
        Ok(args) => Database::from_csv(
            &args.output_file,
            CsvFiles::new(
                read_to_cow(&args.frequency_file)?,
                read_to_cow(&args.conversions_file)?,
            ),
        ),
        Err(e) => {
            log::error!("{}", e);
            return Err(e);
        },
    };

    if let Err(e) = result {
        log::error!("{}", e);
        return Err(e.into());
    }

    Ok(())
}

fn read_to_cow(file: &str) -> Result<Cow<str>> {
    Ok(Cow::Owned(read_to_string(file)?))
}
