use std::borrow::Cow;
use std::collections::HashSet;
use std::hash::Hash;

use anyhow::Result;
use csv::Reader;
use serde::Deserialize;

use crate::db::models::Conversion;
use crate::db::models::Input;
use crate::db::models::InputLookup;

pub struct CsvFiles<'a> {
    pub input_csv: Cow<'a, str>,
    pub conversion_csv: Cow<'a, str>,
}

impl<'a> CsvFiles<'a> {
    pub fn new(csv_data: Cow<'a, str>, conversion_data: Cow<'a, str>) -> Self {
        Self {
            input_csv: csv_data,
            conversion_csv: conversion_data,
        }
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, Hash)]
pub struct CsvFrequency {
    pub input: String,
    #[serde(rename = "freq")]
    pub corpus_count: i64,
    pub chhan_id: i64,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Hash)]
pub struct CsvConversion {
    pub input: String,
    pub output: String,
    pub weight: i64,
    #[serde(rename = "hint")]
    pub annotation: Option<String>,
    #[serde(rename = "color")]
    pub category: Option<i64>,
}

fn load_freq_records(csv_data: Cow<str>) -> Result<(Vec<CsvFrequency>, i64)> {
    let reader = Reader::from_reader(csv_data.as_bytes());
    let mut total_count = 0;
    let mut records = vec![];

    for result in reader.into_deserialize() {
        let result: CsvFrequency = result?;
        total_count += result.corpus_count;
        records.push(result);
    }

    Ok((records, total_count))
}

pub fn frequencies_from_csv(
    csv_data: Cow<str>,
) -> Result<(Vec<Input>, InputLookup)> {
    let (records, total_count) = load_freq_records(csv_data)?;

    let mut input_lookup = InputLookup::new();
    let mut inputs = vec![];
    let mut id = 1;

    for row in records.into_iter() {
        let CsvFrequency {
            input,
            corpus_count,
            chhan_id,
        } = row;

        let n_syls = input.split(" ").count();
        let p = (corpus_count as f64) / (total_count as f64);

        let record = Input {
            id,
            input,
            corpus_count,
            chhan_id,
            n_syls,
            p,
        };

        let key = record.input.clone();

        if !input_lookup.contains_input(&key) {
            input_lookup.insert(&record);
            inputs.push(record);
            id += 1;
        }
    }

    Ok((inputs, input_lookup))
}

pub fn conversions_from_csv(
    csv_data: Cow<str>,
    input_lookup: &InputLookup,
) -> Result<Vec<Conversion>> {
    let mut seen = HashSet::new();
    let mut records = vec![];

    let reader = Reader::from_reader(csv_data.as_bytes());
    let mut n = 0;

    for result in reader.into_deserialize() {
        let CsvConversion {
            input,
            output,
            weight,
            annotation,
            category,
        } = result?;

        if let Some(input_id) = input_lookup.id_of(&input) {
            if seen.insert((input.clone(), output.clone())) {
                n += 1;
                records.push(Conversion {
                    input_id,
                    output,
                    weight,
                    annotation,
                    category,
                })
            } else {
                log::debug!(
                    "Duplicate conversion: {:?}, {:?}",
                    input.clone(),
                    output.clone()
                );
            }
        } else {
            // log::debug!("No input found for conversion from: {:?}", input);
        }
    }

    log::debug!("Total conversions: {}", n);

    Ok(records)
}
