use anyhow::Result;

use crate::buffer::Buffer;
use crate::buffer::BufferElement;
use crate::buffer::BufferElementEnum;
use crate::buffer::KhiinElem;
use crate::buffer::StringElem;
use crate::config::Config;
use crate::data::Database;
use crate::data::Dictionary;
use crate::engine::EngInner;
use crate::input::parser::SectionType;

use super::parse_longest_from_start;
use super::parse_whole_input;

pub(crate) fn get_candidates(
    engine: &EngInner,
    raw_buffer: &str,
) -> Result<Vec<Buffer>> {
    let (ty, query) = parse_longest_from_start(&engine.dict, raw_buffer);

    match ty {
        SectionType::Plaintext => Ok(Vec::new()),
        SectionType::Hyphens => Ok(Vec::new()),
        SectionType::Punct => Ok(Vec::new()),
        SectionType::Splittable => candidates_for_splittable(engine, query),
    }
}

fn candidates_for_splittable(
    engine: &EngInner,
    query: &str,
) -> Result<Vec<Buffer>> {
    let EngInner { db, dict, conf } = &engine;
    let mut words = dict.all_words_from_start(query);
    words.retain(|&w| {
        if let Some(rem) = query.strip_prefix(w) {
            dict.can_segment(rem)
        } else {
            true
        }
    });

    let candidates =
        db.find_conversions_for_words(conf.input_type(), &words)?;

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
    engine: &EngInner,
    raw_buffer: &str,
) -> Result<Buffer> {
    let sections = parse_whole_input(&engine.dict, raw_buffer);
    let mut composition = Buffer::new();

    for (ty, section) in sections {
        match ty {
            SectionType::Plaintext => {
                composition.push(StringElem::from(section).into());
            },
            SectionType::Hyphens => todo!(),
            SectionType::Punct => todo!(),
            SectionType::Splittable => {
                let elems = convert_section(engine, ty, section)?;
                for elem in elems.into_iter() {
                    composition.push(elem)
                }
            },
        }
    }

    Ok(composition)
}

fn convert_section(
    engine: &EngInner,

    ty: SectionType,
    section: &str,
) -> Result<Vec<BufferElementEnum>> {
    let mut ret = Vec::new();

    let words = engine.dict.segment(section)?;
    for word in words {
        let conversions = engine.db.find_conversions(
            engine.conf.input_type(),
            word.as_str(),
            Some(1),
        )?;

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
        let (engine, _) = test_harness();
        let comp = convert_all(&engine, "abc");
        log::debug!("{:#?}", comp);
    }

    #[test]
    fn it_gets_candidates() -> Result<()> {
        let (engine, _) = test_harness();
        let cands = get_candidates(&engine, "a")?;
        log::debug!("{:#?}", cands);
        Ok(())
    }

    #[test_log::test]
    fn it_contains_ia7() -> Result<()> {
        let (engine, _) = test_harness();
        let result = candidates_for_splittable(&engine, "ia7")?;
        assert!(result.iter().any(|c| c.display_text() == "掖"));
        Ok(())
    }
}
