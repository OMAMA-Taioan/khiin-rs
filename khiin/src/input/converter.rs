use anyhow::Result;

use crate::config::Config;
use crate::data::Database;
use crate::data::Dictionary;
use crate::input::parser::SectionType;

use super::parse_input;

pub fn convert_all(
    db: &Database,
    dict: &Dictionary,
    cfg: &Config,
    raw_buffer: &str,
) -> Result<()> {
    let sections = parse_input(dict, raw_buffer);

    for section in sections {
        match section.ty {
            SectionType::Unknown => todo!(),
            SectionType::Hyphens => todo!(),
            SectionType::Punct => todo!(),
            SectionType::Splittable => {
                let words = dict.segment(section.text)?;
                for word in words {
                    let candidates =
                        db.find_conversions(cfg.input_type(), raw_buffer)?;
                }
            },
        }
    }

    Ok(())
}
