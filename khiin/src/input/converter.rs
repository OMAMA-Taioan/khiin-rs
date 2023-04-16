use anyhow::Result;

use crate::buffer::Buffer;
use crate::buffer::BufferElementEnum;
use crate::buffer::KhiinElem;
use crate::buffer::StringElem;
use crate::config::Config;
use crate::data::Database;
use crate::data::Dictionary;
use crate::input::parser::SectionType;

use super::parse_longest_from_start;
use super::parse_whole_input;

pub(crate) fn get_candidates(
    db: &Database,
    dict: &Dictionary,
    conf: &Config,
    raw_buffer: &str,
) -> Result<Vec<Buffer>> {
    let (ty, query) = parse_longest_from_start(dict, raw_buffer);

    match ty {
        SectionType::Plaintext => Ok(Vec::new()),
        SectionType::Hyphens => Ok(Vec::new()),
        SectionType::Punct => Ok(Vec::new()),
        SectionType::Splittable => {
            let words = dict.all_words_from_start(query);
            let candidates =
                db.find_conversions_for_ids(conf.input_type(), &words)?;

            let mut result: Vec<Buffer> = Vec::new();

            for conv in candidates {
                let khiin_elem: BufferElementEnum =
                    KhiinElem::from_conversion(&conv.key_sequence, &conv)?.into();
                let mut buf: Buffer = khiin_elem.into();
                buf.set_converted(true);
                result.push(buf);
            }

            Ok(result)
        },
    }
}

pub(crate) fn convert_all(
    db: &Database,
    dict: &Dictionary,
    cfg: &Config,
    raw_buffer: &str,
) -> Result<Buffer> {
    let sections = parse_whole_input(dict, raw_buffer);
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
                        let khiin_elem =
                            KhiinElem::from_conversion(raw_buffer, conv)?;
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
    use super::*;
    use crate::config::*;
    use crate::data::*;
    use crate::tests::*;

    fn setup() -> (Database, Dictionary, Config) {
        (get_db(), get_dict(), get_conf())
    }

    #[test]
    fn it_splits_and_converts_words() {
        let (db, dict, conf) = setup();
        let comp = convert_all(&db, &dict, &conf, "abc");
        println!("{:#?}", comp);
    }

    #[test]
    fn it_gets_candidates() -> Result<()> {
        let (db, dict, conf) = setup();
        let cands = get_candidates(&db, &dict, &conf, "a")?;
        // println!("{:#?}", cands);
        Ok(())
    }
}
