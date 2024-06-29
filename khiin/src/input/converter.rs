use anyhow::Result;

use crate::buffer::Buffer;
use crate::buffer::BufferElement;
use crate::buffer::BufferElementEnum;
use crate::buffer::KhiinElem;
use crate::buffer::StringElem;
use crate::config;
use crate::config::Config;
use crate::data::Dictionary;
use crate::db::Database;
use crate::engine::EngInner;
use crate::input::parser::SectionType;

use super::parse_longest_from_start;
use super::parse_whole_input;
use super::Syllable;

use khiin_ji::lomaji::has_tone_letter;
use khiin_ji::lomaji::strip_tone_diacritic;
use khiin_ji::Tone;

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
        db.select_conversions_for_multiple(conf.tone_mode().into(), &words)?;

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

pub(crate) fn convert_to_telex(
    engine: &EngInner,
    raw_buffer: &str,
    key: char,
) -> Result<Buffer> {
    let (stripped, tone) = strip_tone_diacritic(raw_buffer);

    let mut word: Syllable = Syllable::new();
    word.raw_body = stripped.to_string();
    word.raw_input = stripped.to_string();
    
    let mut tone_char: char = key.to_ascii_lowercase();
    if tone != Tone::T1 || has_tone_letter(raw_buffer) {
        if (tone_char == engine.conf.t2()) {
            word.tone = Tone::T2;
        } else if (tone_char == engine.conf.t3()) {
            word.tone = Tone::T3
        } else if (tone_char == engine.conf.t5()) {
            word.tone = Tone::T5
        } else if (tone_char == engine.conf.t6()) {
            word.tone = Tone::T6
        } else if (tone_char == engine.conf.t7()) {
            word.tone = Tone::T7
        } else if (tone_char == engine.conf.t9()) {
            word.tone = Tone::T9
        } else if (tone_char == engine.conf.t8()) {
            word.tone = Tone::T8
        } else if (tone_char == engine.conf.khin()) {
            tone_char = get_tone_char(engine, &tone);
            word.tone = tone;
            word.khin = true;
        } else {
            tone_char = get_tone_char(engine, &tone);
            word.tone = tone;
            if key != ' ' {
                word.raw_body.push(key);
            }
        }
        if (tone_char == engine.conf.t8()) {
            // shared T8 key
            word.tone = get_shared_t8_tone(engine);
            if word.raw_body.ends_with(&['p', 't', 'k', 'h', 'P', 'T', 'K', 'H']) {
                word.tone = Tone::T8;
            }
        }
    } else {
        if key != ' ' {
            word.raw_body.push(key);
        }
    }
    if (word.tone == Tone::T1) {
        word.tone = Tone::None
    }
    let mut composition = Buffer::new();
    composition.push(StringElem::from(word.compose()).into());
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
        let conversions = engine.db.select_conversions(
            engine.conf.tone_mode().into(),
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

fn get_tone_char(engine: &EngInner, tone: &Tone) -> char {
    match tone {
        Tone::None => ' ',
        Tone::T1 => ' ',
        Tone::T2 => engine.conf.t2(),
        Tone::T3 => engine.conf.t3(),
        Tone::T4 => ' ',
        Tone::T5 => engine.conf.t5(),
        Tone::T6 => engine.conf.t6(),
        Tone::T7 => engine.conf.t7(),
        Tone::T8 => engine.conf.t8(),
        Tone::T9 => engine.conf.t9(),
    }
}

fn get_shared_t8_tone(engine: &EngInner) -> Tone {
    let t8_char = engine.conf.t8();
    if (t8_char == engine.conf.t2()) {
        return Tone::T2;
    } else if (t8_char == engine.conf.t3()) {
        return Tone::T3;
    } else if (t8_char == engine.conf.t5()) {
        return Tone::T5;
    } else if (t8_char == engine.conf.t6()) {
        return Tone::T6;
    } else if (t8_char == engine.conf.t7()) {
        return Tone::T7;
    } else if (t8_char == engine.conf.t9()) {
        return Tone::T9;
    }
    return Tone::T8;
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
