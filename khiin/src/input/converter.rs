use anyhow::Result;

use crate::buffer::Buffer;
use crate::buffer::BufferElement;
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
            candidates_for_splittable(db, dict, conf, query)
        },
    }
}

fn candidates_for_splittable(
    db: &Database,
    dict: &Dictionary,
    conf: &Config,
    query: &str,
) -> Result<Vec<Buffer>> {
    let mut words = dict.all_words_from_start(query);
    words.retain(|&w| {
        if let Some(rem) = query.strip_prefix(w) {
            dict.can_segment(rem)
        } else {
            true
        }
    });
    
    let candidates = db.find_conversions_for_ids(conf.input_type(), &words)?;

    let result = candidates
        .into_iter()
        .map(|conv| KhiinElem::from_conversion(&conv.key_sequence, &conv))
        .filter(|elem| elem.is_ok())
        .map(|elem| elem.unwrap().into())
        .filter(|elem: &BufferElementEnum| {
            let len = elem.raw_text().len();
            len >= query.len() || dict.can_segment(&query[len..])
        })
        .map(|elem| {
            let mut buffer: Buffer = elem.into();
            buffer.set_converted(true);
            buffer
        })
        .collect();

    Ok(result)
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
                let elems = convert_section(db, dict, cfg, ty, section)?;
                for elem in elems.into_iter() {
                    composition.push(elem)
                }
            },
        }
    }

    Ok(composition)
}

fn convert_section(
    db: &Database,
    dict: &Dictionary,
    cfg: &Config,
    ty: SectionType,
    section: &str,
) -> Result<Vec<BufferElementEnum>> {
    let mut ret = Vec::new();

    let words = dict.segment(section)?;
    for word in words {
        let conversions =
            db.find_conversions(cfg.input_type(), word.as_str(), Some(1))?;

        if let Some(conv) = conversions.get(0) {
            let khiin_elem = KhiinElem::from_conversion(&word, conv)?;
            ret.push(khiin_elem.into());
        }
    }

    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    fn setup() -> (Database, Dictionary, Config) {
        (get_db(), get_dict(), get_conf())
    }

    #[test]
    fn it_splits_and_converts_words() {
        let (db, dict, conf) = setup();
        let comp = convert_all(&db, &dict, &conf, "abc");
        log::debug!("{:#?}", comp);
    }

    #[test]
    fn it_gets_candidates() -> Result<()> {
        let (db, dict, conf) = setup();
        let cands = get_candidates(&db, &dict, &conf, "a")?;
        log::debug!("{:#?}", cands);
        Ok(())
    }

    #[test_log::test]
    fn it_contains_ia7() -> Result<()> {
        let (db, dict, conf) = setup();
        let result = candidates_for_splittable(&db, &dict, &conf, "ia7")?;
        assert!(result.iter().any(|c| c.display_text() == "掖"));
        Ok(())
    }
}
