use anyhow::Result;

use crate::buffer::Buffer;
use crate::buffer::KhiinElem;
use crate::buffer::StringElem;
use crate::config::Config;
use crate::data::Database;
use crate::data::Dictionary;
use crate::input::parser::SectionType;

use super::parse_input;

pub(crate) fn find_conversion_candidates(
    db: &Database,
    dict: &Dictionary,
    cfg: &Config,
    raw_buffer: &str,
) -> Result<Vec<Buffer>> {
    Ok(Vec::new())
}

pub(crate) fn convert_all(
    db: &Database,
    dict: &Dictionary,
    cfg: &Config,
    raw_buffer: &str,
) -> Result<(Buffer, Vec<Buffer>)> {
    let sections = parse_input(dict, raw_buffer);
    let mut composition = Buffer::new();

    for section in sections {
        match section.ty {
            SectionType::Unknown => {
                composition.push(StringElem::from(section.raw_buffer).into());
            },
            SectionType::Hyphens => todo!(),
            SectionType::Punct => todo!(),
            SectionType::Splittable => {
                let words = dict.segment(section.raw_buffer)?;
                for word in words {
                    let conversions = db.find_conversions(
                        cfg.input_type(),
                        word.as_str(),
                        Some(1),
                    )?;
                    if let Some(conv) = conversions.get(0) {
                        let khiin_elem = KhiinElem::from_conversion(
                            raw_buffer,
                            conv,
                        )?;
                        composition.push(khiin_elem.into());
                    }
                }
            },
        }
    }

    let candidates = find_conversion_candidates(db, dict, cfg, raw_buffer)?;
    Ok((composition, candidates))
}
