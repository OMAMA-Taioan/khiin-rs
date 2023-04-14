use anyhow::Result;

use crate::buffer::Buffer;
use crate::buffer::KhiinElem;
use crate::config::Config;
use crate::data::Database;
use crate::data::Dictionary;
use crate::input::parser::SectionType;

use super::parse_input;

pub(crate) fn convert_all(
    db: &Database,
    dict: &Dictionary,
    cfg: &Config,
    raw_buffer: &str,
) -> Result<(Buffer, Vec<Buffer>)> {
    let sections = parse_input(dict, raw_buffer);
    let mut composition = Buffer::new();
    let mut candidates = Vec::new();

    for section in sections {
        match section.ty {
            SectionType::Unknown => {
                // TODO
            },
            SectionType::Hyphens => todo!(),
            SectionType::Punct => todo!(),
            SectionType::Splittable => {
                let words = dict.segment(section.text)?;
                for word in words {
                    let conversions =
                        db.find_conversions(cfg.input_type(), word.as_str())?;
                    let tai_text = KhiinElem::from_conversion(
                        raw_buffer,
                        &conversions[0],
                    )?;
                    composition.push(tai_text.into());
                }
            },
        }
    }

    Ok((composition, candidates))
}
