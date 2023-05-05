use std::borrow::Cow;

use anyhow::Result;
use rusqlite::params;
use rusqlite::Connection;

use crate::db::models::generate_key_sequences;
use crate::db::models::Conversion;
use crate::db::models::Input;
use crate::db::models::KeySequence;

use super::csv::conversions_from_csv;
use super::csv::frequencies_from_csv;
use super::csv::CsvFiles;

pub(crate) fn collect_data(
    csv_files: CsvFiles,
) -> Result<(Vec<Input>, Vec<Conversion>, Vec<KeySequence>)> {
    let CsvFiles {
        input_csv,
        conversion_csv,
    } = csv_files;

    let (inputs, input_lookup) = frequencies_from_csv(input_csv)?;
    let conversions = conversions_from_csv(conversion_csv, &input_lookup)?;
    let key_sequences = generate_key_sequences(&inputs)?;

    log::debug!("Total key sequences: {}", key_sequences.len());

    Ok((inputs, conversions, key_sequences))
}

pub(crate) fn insert_inputs(
    conn: &mut Connection,
    inputs: Vec<Input>,
) -> Result<()> {
    let tx = conn.transaction()?;
    let mut stmt =
        tx.prepare(include_str!("../sql/insert_inputs.sql"))?;

    for input in inputs {
        stmt.execute(params![
            input.id,
            input.input,
            input.corpus_count,
            input.chhan_id,
        ])?;
    }

    drop(stmt);
    tx.commit()?;

    Ok(())
}

pub(crate) fn insert_conversions(
    conn: &mut Connection,
    conversions: Vec<Conversion>,
) -> Result<()> {
    let tx = conn.transaction()?;
    let mut stmt =
        tx.prepare(include_str!("../sql/insert_conversions.sql"))?;

    for row in conversions {
        stmt.execute(params![
            row.input_id,
            row.output,
            row.weight,
            row.category,
            row.annotation,
        ])?;
    }

    drop(stmt);
    tx.commit()?;

    Ok(())
}

pub(crate) fn insert_key_sequences(
    conn: &mut Connection,
    rows: Vec<KeySequence>,
) -> Result<()> {
    let tx = conn.transaction()?;
    let mut stmt =
        tx.prepare(include_str!("../sql/insert_key_sequences.sql"))?;

    for row in rows {
        stmt.execute(params![
            row.input_id,
            row.keys,
            row.input_type as i64,
            row.n_syls,
            row.p,
        ])?;
    }

    drop(stmt);
    tx.commit()?;

    Ok(())
}

pub(crate) fn build_sql(conn: &mut Connection) -> Result<()> {
    let csv_files = CsvFiles::new(
        Cow::Borrowed(khiin_data::INPUTS_CSV),
        Cow::Borrowed(khiin_data::CONVERSIONS_CSV),
    );

    build_sql_from_csv(conn, csv_files)
}

pub(crate) fn build_sql_from_csv(
    conn: &mut Connection,
    csv_files: CsvFiles,
) -> Result<()> {
    let (inputs, mut conversions, mut key_sequences) = collect_data(csv_files)?;

    conversions.sort_by(|a, b| a.input_id.cmp(&b.input_id));
    key_sequences.sort_by(|a, b| a.input_id.cmp(&b.input_id));

    insert_inputs(conn, inputs)?;
    insert_conversions(conn, conversions)?;
    insert_key_sequences(conn, key_sequences)?;
    Ok(())
}
