use anyhow::Result;
use itertools::Itertools;

use crate::db::models::Input;
use khiin_ji::poj_syl_to_key_sequences;
use khiin_ji::poj_syl_to_key_sequences_oo;
use rusqlite::types::FromSql;
use rusqlite::types::FromSqlResult;
use rusqlite::types::ToSqlOutput;
use rusqlite::types::ValueRef;
use rusqlite::ToSql;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(i64)]
// TODO
// Write comment to explain InputType
pub enum InputType {
    Detoned = 0,
    Numeric = 1,
    Telex = 2,
}

impl InputType {
    fn as_i64(&self) -> i64 {
        *self as i64
    }
}

impl ToSql for InputType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.as_i64()))
    }
}

impl FromSql for InputType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let ty = match value.as_i64()? {
            1 => InputType::Numeric,
            2 => InputType::Telex,
            _ => InputType::Detoned,
        };
        FromSqlResult::from(Ok(ty))
    }
}

#[derive(Debug, PartialEq)]
pub struct KeySequence {
    pub input_id: i64,
    pub keys: String,
    pub input_type: InputType,
    pub n_syls: usize,
    pub p: f64,
}

impl KeySequence {
    pub fn of_single_syl_set(
        numeric: String,
        telex: String,
        toneless: String,
        input: &Input,
    ) -> Vec<Self> {
        vec![
            Self {
                keys: numeric,
                n_syls: 1,
                input_type: InputType::Numeric,
                input_id: input.id,
                p: input.p,
            },
            Self {
                keys: telex,
                n_syls: 1,
                input_type: InputType::Telex,
                input_id: input.id,
                p: input.p,
            },
            Self {
                keys: toneless,
                n_syls: 1,
                input_type: InputType::Detoned,
                input_id: input.id,
                p: input.p,
            },
        ]
    }

    pub fn of_multi_syl_set(
        numeric: Vec<String>,
        telex: Vec<String>,
        detoned: Vec<String>,
        input: &Input,
    ) -> Vec<Self> {
        let mut result = vec![];

        for keys in numeric {
            result.push(KeySequence {
                keys,
                input_type: InputType::Numeric,
                n_syls: input.n_syls,
                input_id: input.id,
                p: input.p,
            });
        }

        for keys in telex {
            result.push(KeySequence {
                keys,
                input_type: InputType::Telex,
                n_syls: input.n_syls,
                input_id: input.id,
                p: input.p,
            });
        }

        for keys in detoned {
            result.push(KeySequence {
                keys,
                input_type: InputType::Detoned,
                n_syls: input.n_syls,
                input_id: input.id,
                p: input.p,
            });
        }

        result
    }
}

pub fn generate_key_sequences(inputs: &Vec<Input>) -> Result<Vec<KeySequence>> {
    Ok(inputs
        .iter()
        .flat_map(|input| {
            if let Ok(key_sequences) = generate_key_sequence(input) {
                key_sequences
            } else {
                log::debug!("Problem with key_sequence for: {:?}", input);
                Vec::new()
            }
        })
        .collect::<Vec<KeySequence>>())
}

fn generate_key_sequence(input: &Input) -> Result<Vec<KeySequence>> {
    if input.n_syls == 1 {
        let (numeric, telex, detoned) = poj_syl_to_key_sequences(&input.input);
        let (numeric_oo, telex_oo, detoned_oo) = poj_syl_to_key_sequences_oo(&input.input);
        if detoned_oo == detoned {
            return Ok(KeySequence::of_single_syl_set(
                numeric, telex, detoned, input,
            ));
        } else {
            let mut result = KeySequence::of_single_syl_set(
                numeric, telex, detoned, input,
            );
            result.extend(KeySequence::of_single_syl_set(
                numeric_oo, telex_oo, detoned_oo, input,
            ));
            return Ok(result);
        }
    }

    let mut numeric_syls: Vec<Vec<String>> = vec![];
    let mut telex_syls: Vec<Vec<String>> = vec![];
    let mut detoned_syls: Vec<Vec<String>> = vec![];

    input.input.split(" ").for_each(|syl| {
        let (numeric, telex, detoned) = poj_syl_to_key_sequences(syl);

        if syl.contains("\u{0358}") {
            let (numeric_oo, telex_oo, detoned_oo) = poj_syl_to_key_sequences_oo(syl);

            numeric_syls.push(vec![numeric, numeric_oo, detoned.clone(), detoned_oo.clone()]);
            telex_syls.push(vec![telex, telex_oo, detoned.clone(), detoned_oo.clone()]);
            detoned_syls.push(vec![detoned, detoned_oo]);
        } else {
            numeric_syls.push(vec![numeric, detoned.clone()]);
            telex_syls.push(vec![telex, detoned.clone()]);
            detoned_syls.push(vec![detoned]);
        }
    });

    let numeric = multi_cartesian_product(numeric_syls);
    let telex = multi_cartesian_product(telex_syls);
    let detoned = multi_cartesian_product(detoned_syls);

    Ok(KeySequence::of_multi_syl_set(numeric, telex, detoned, input))
}

fn multi_cartesian_product(constituents: Vec<Vec<String>>) -> Vec<String> {
    let mut vec = Vec::new();
    for p in constituents.into_iter().multi_cartesian_product() {
        vec.push(p.into_iter().collect::<String>());
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::models::*;

    fn key_seq(
        input_id: i64,
        keys: &str,
        input_type: InputType,
        n_syls: usize,
    ) -> KeySequence {
        KeySequence {
            input_id,
            keys: keys.to_string(),
            n_syls,
            input_type,
            p: 0.0,
        }
    }

    fn input(input: &str, id: i64) -> Input {
        let n_syls = input.split(" ").count();
        Input {
            id,
            input: input.to_string(),
            n_syls,
            ..Default::default()
        }
    }

    #[test_log::test]
    fn it_makes_key_seqeuences() {
        let input_1 = input("á bô", 1);
        let input_2 = input("ò", 2);
        let input_3 = input("hô͘ tô͘", 3);

        let mut data_helper = InputLookup::default();
        data_helper.insert(&input_1);
        data_helper.insert(&input_2);
        data_helper.insert(&input_3);

        let result = generate_key_sequence(&input_1);
        assert!(result.is_ok());
        let result = result.unwrap();
        let expect = vec![
            key_seq(1, "a2bo5", InputType::Numeric, 2),
            key_seq(1, "abo5", InputType::Numeric, 2),
            key_seq(1, "a2bo", InputType::Numeric, 2),
            key_seq(1, "abo", InputType::Numeric, 2),
            key_seq(1, "asbol", InputType::Telex, 2),
            key_seq(1, "abol", InputType::Telex, 2),
            key_seq(1, "asbo", InputType::Telex, 2),
            key_seq(1, "abo", InputType::Telex, 2),
        ];
        assert_eq!(result.len(), 9);
        for item in &expect {
            assert!(result.contains(item));
        }

        let result = generate_key_sequence(&input_2);
        assert!(result.is_ok());
        let result = result.unwrap();
        let expect = vec![
            key_seq(2, "o3", InputType::Numeric, 1),
            key_seq(2, "of", InputType::Telex, 1),
            key_seq(2, "o", InputType::Detoned, 1),
        ];
        for item in &expect {
            assert!(result.contains(item));
        }

        let result = generate_key_sequence(&input_3);
        assert!(result.is_ok());
        let result = result.unwrap();
        let expect = vec![
            key_seq(3, "hootoo5", InputType::Numeric, 2),
            key_seq(3, "hootool", InputType::Telex, 2),
            key_seq(3, "hootoo", InputType::Detoned, 2),
            key_seq(3, "houtou5", InputType::Numeric, 2),
            key_seq(3, "houtoul", InputType::Telex, 2),
            key_seq(3, "houtou", InputType::Detoned, 2),
        ];
        for item in &expect {
            assert!(result.contains(item));
        }

        let result = generate_key_sequences(&vec![input_1, input_2]).unwrap();
        assert_eq!(result.len(), 12);
    }
}
