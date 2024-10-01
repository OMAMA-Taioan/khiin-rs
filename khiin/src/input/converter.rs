use anyhow::Result;

use crate::buffer::Buffer;
use crate::buffer::BufferElement;
use crate::buffer::BufferElementEnum;
use crate::buffer::KhiinElem;
use crate::buffer::StringElem;
use crate::config::Config;
use crate::config::ToneMode;
use crate::config::OutputMode;
use crate::data::Dictionary;
use crate::db::models::InputType;
use crate::db::models::CaseType;
use crate::db::Database;
use crate::engine::EngInner;
use crate::input::parser::SectionType;

use super::parse_longest_from_start;
use super::parse_whole_input;
use super::Syllable;

use khiin_ji::lomaji::has_tone_letter;
use khiin_ji::lomaji::strip_khin;
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

fn get_case_type (text: &str) -> CaseType {
    if text.is_empty() {
        return CaseType::Lowercase;
    }
    if text.chars().all(|c| c.is_uppercase()) {
        CaseType::Uppercase
    } else if let Some(first_char) = text.chars().next() {
        if first_char.is_uppercase() {
            CaseType::FirstUpper
        } else {
            CaseType::Lowercase
        }
    } else {
        CaseType::Lowercase
    }
} 

pub(crate) fn get_candidates_for_word(
    engine: &EngInner,
    query: &str,
) -> Result<Vec<Buffer>> {
    let EngInner { db, dict, conf } = &engine;
    let raw_input = query.to_string().to_ascii_lowercase();
    let case_type = get_case_type(query);
    let candidates =
        db.select_conversions_for_tone(InputType::Detoned, raw_input.as_str(), conf.is_hanji_first())?;

    let result = candidates
        .into_iter()
        .map(|mut conv| {
            conv.set_output_case_type(case_type.clone());
            KhiinElem::from_conversion(&conv.key_sequence, &conv)
        })
        .filter(|elem| elem.is_ok())
        .map(|elem| elem.unwrap().into())
        .filter(|elem: &BufferElementEnum| {
            let len = elem.raw_text().len();
            len >= raw_input.len() || dict.can_segment(&raw_input[len..])
        })
        .map(|elem| {
            let mut buffer: Buffer = elem.into();
            buffer.set_converted(true);
            buffer
        })
        .collect();

    Ok(result)
}

pub(crate) fn get_candidates_for_word_with_tone(
    engine: &EngInner,
    query: &str,
    tone_char: char,
) -> Result<Vec<Buffer>> {
    let EngInner { db, dict, conf } = &engine;
    let mut tone_key = get_numberic_tone_char(engine, tone_char);
    if (tone_char == engine.conf.t8() && get_shared_t8_tone(engine) != Tone::T8) {
        // shared T8 key
        let lower_str = query
            .to_lowercase()
            .replace("ⁿ", "")
            .replace("ᴺ", "")
            .replace("nn", "");
        if lower_str.ends_with(&['p', 't', 'k', 'h']) {
            tone_key = '8'
        }
    }
    let raw_input = query.to_string().to_ascii_lowercase();
    let tone_input = format!("{}{}", raw_input, tone_key);
    let case_type = get_case_type(query);
    let candidates = if (tone_char == '1' || tone_char == '4') {
        db.select_conversions_for_word(InputType::Numeric, tone_input.as_str(), raw_input.as_str(), conf.is_hanji_first())?
    } else {
        db.select_conversions_for_tone(InputType::Numeric, tone_input.as_str(), conf.is_hanji_first())?
    };

    let result = candidates
        .into_iter()
        .map(|mut conv| {
            conv.set_output_case_type(case_type.clone());
            KhiinElem::from_conversion(&conv.key_sequence, &conv)
        })
        .filter(|elem| elem.is_ok())
        .map(|elem| elem.unwrap().into())
        .filter(|elem: &BufferElementEnum| {
            let len = elem.raw_text().len();
            len >= raw_input.len() || dict.can_segment(&raw_input[len..])
        })
        .map(|elem: BufferElementEnum| {
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
) -> (Result<Buffer>, bool) {
    let (stripped, tone) = strip_tone_diacritic(raw_buffer);

    let pre_tone_char = tone_to_char(engine, &tone);
    if pre_tone_char != ' ' && pre_tone_char == key.to_ascii_lowercase() {
        // duplicate tone characters
        let mut composition = Buffer::new();
        let mut raw_input = stripped.to_string();
        raw_input.push(key);
        composition.push(StringElem::from(raw_input).into());
        return (Ok(composition), false);
    } else if (raw_buffer.starts_with("--")
        && key.to_ascii_lowercase() == engine.conf.khin())
    {
        // duplicate khin characters
        let mut composition = Buffer::new();
        let mut raw_input = stripped.to_string();
        raw_input.drain(0..2);
        raw_input.push(key);
        composition.push(StringElem::from(raw_input).into());
        return (Ok(composition), false);
    }
    let mut word: Syllable = Syllable::new();
    word.raw_body = stripped.to_string();
    word.raw_input = stripped.to_string();
    word.raw_input.push(key);

    if tone != Tone::T1 || has_tone_letter(raw_buffer) {
        let mut tone_char: char = key.to_ascii_lowercase();
        word.tone = char_to_tone(engine, tone_char);
        if (word.tone == Tone::None) {
            if (tone_char == engine.conf.khin()) {
                tone_char = tone_to_char(engine, &tone);
                word.tone = tone;
                word.khin = true;
            } else {
                tone_char = tone_to_char(engine, &tone);
                word.tone = tone;
                if key != ' ' {
                    word.raw_body.push(key);
                }
            }
        }

        if (tone_char == engine.conf.t8()) {
            // shared T8 key
            word.tone = get_shared_t8_tone(engine);
            let lower_str = word
                .raw_body
                .to_lowercase()
                .replace("ⁿ", "")
                .replace("ᴺ", "")
                .replace("nn", "");
            if lower_str.ends_with(&['p', 't', 'k', 'h']) {
                word.tone = Tone::T8;
            }
        }
    } else {
        if key != ' ' {
            word.raw_body.push(key);
        }
    }
    let syllable = word.compose();
    let (mut stripped, tone) = strip_tone_diacritic(&syllable);
    _ = strip_khin(&mut stripped);
    stripped = stripped.replace("O͘", "o͘").replace("ᴺ", "ⁿ");
    let ret = engine.dict.is_illegal_syllable_prefix(&stripped);

    let mut composition = Buffer::new();
    composition.push(StringElem::from(syllable).into());
    (Ok(composition), ret)
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
            let khiin_elem: KhiinElem =
                KhiinElem::from_conversion(&word, conv)?;
            ret.push(khiin_elem.into());
        }
    }

    Ok(ret)
}
pub(crate) fn get_numberic_tone_char(engine: &EngInner, ch: char) -> char {
    if (engine.conf.tone_mode() == ToneMode::Telex) {
        let tone = char_to_tone(engine, ch);
        match tone {
            Tone::None => ch,
            Tone::T1 => ch,
            Tone::T2 => '2',
            Tone::T3 => '3',
            Tone::T4 => ch,
            Tone::T5 => '5',
            Tone::T6 => '6',
            Tone::T7 => '7',
            Tone::T8 => '8',
            Tone::T9 => '9',
        }
    } else {
        ch
    }
}

fn tone_to_char(engine: &EngInner, tone: &Tone) -> char {
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

fn char_to_tone(engine: &EngInner, ch: char) -> Tone {
    if ch == engine.conf.t2() {
        Tone::T2
    } else if ch == engine.conf.t3() {
        Tone::T3
    } else if ch == engine.conf.t5() {
        Tone::T5
    } else if ch == engine.conf.t6() {
        Tone::T6
    } else if ch == engine.conf.t7() {
        Tone::T7
    } else if ch == engine.conf.t9() {
        Tone::T9
    } else if ch == engine.conf.t8() {
        Tone::T8
    } else {
        Tone::None
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
