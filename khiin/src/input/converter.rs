use anyhow::Result;

use crate::buffer::Buffer;
use crate::buffer::KhiinElem;
use crate::buffer::StringElem;
use crate::config::Config;
use crate::data::Database;
use crate::data::Dictionary;
use crate::input::parser::SectionType;

use super::parse_input;

pub(crate) fn get_candidates(
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
) -> Result<Buffer> {
    let sections = parse_input(dict, raw_buffer);
    let mut composition = Buffer::new();

    for (ty, section) in sections {
        match ty {
            SectionType::Plaintext => {
                composition.push(StringElem::from(section).into());
            },
            SectionType::Hyphens => todo!(),
            SectionType::Punct => todo!(),
            SectionType::Splittable => {
                let words = dict.segment(section)?;
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

    Ok(composition)
}

#[cfg(test)]
mod tests {
    use crate::config::*;
    use crate::data::*;
    use crate::tests::*;
    use super::*;

    fn setup() -> (Database, Dictionary, Config) {
        (get_db(), get_dict(), get_conf())
    }

    #[test]
    fn it_splits_and_converts_words() {
        let (db, dict, conf) = setup();
        let comp = convert_all(&db, &dict, &conf, "abc");
        println!("{:#?}", comp);
    }
}
