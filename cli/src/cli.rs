use std::io::stdin;
use std::io::stdout;
use std::io::Stdout;
use std::io::Write;

use anyhow::Result;
use khiin_protos::command::Command;
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

use crate::engine_ctrl::EngineCtrl;

fn get_db_filename() -> Result<String> {
    let mut db_path = std::env::current_exe()?;
    db_path.set_file_name("khiin.db");
    Ok(db_path.to_str().unwrap().to_string())
}

fn clear(stdout: &mut RawTerminal<Stdout>) -> Result<()> {
    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        Goto(1, 1),
        termion::cursor::Hide
    )?;
    stdout.flush()?;
    Ok(())
}

fn blank_display(stdout: &mut RawTerminal<Stdout>) -> Result<()> {
    clear(stdout)?;
    update_display(stdout, "", "", &Vec::new())?;
    Ok(())
}

fn update_display(
    stdout: &mut RawTerminal<Stdout>,
    raw: &str,
    display: &str,
    cands: &Vec<String>,
) -> Result<()> {
    let mut display = display;
    if display.is_empty() {
        display = "|";
    }

    clear(stdout)?;
    write!(stdout, "{}Khíín Phah Jī Hoat", Goto(2, 2))?;
    write!(stdout, "{}Raw input:  {}", Goto(2, 4), raw)?;
    write!(stdout, "{}User sees:  {}", Goto(2, 6), display)?;
    write!(stdout, "{}Candidates:  ", Goto(2, 8))?;

    for (i, cand) in cands.iter().enumerate() {
        write!(stdout, "{}{}", Goto(15, 8 + i as u16), cand)?;
    }

    draw_footer(stdout)?;

    stdout.flush()?;
    Ok(())
}

fn page_range(
    item_count: usize,
    page_size: usize,
    index: usize,
) -> (usize, usize) {
    let start = (index / page_size) * page_size;
    let end = std::cmp::min(start + page_size, item_count);
    (start, end)
}

fn get_candidate_page(cmd: &Command) -> Vec<String> {
    let page_size = 9;
    let cl = &cmd.response.candidate_list;
    let item_count = cl.candidates.len();

    let (start, end) = if cl.focused < 0 {
        (0, std::cmp::min(item_count, 9))
    } else {
        page_range(item_count, page_size, cl.focused as usize)
    };

    let mut ret = Vec::new();

    for i in start..end {
        let num = (i % page_size) + 1;
        let mut cand = String::new();
        if i as i32 == cl.focused {
            cand.push_str("*");
        } else {
            cand.push_str(" ");
        }

        cand.push_str(format!("{}. {}", num, cl.candidates[i].value).as_str());
        ret.push(cand)
    }

    ret
}

fn draw_ime(
    stdout: &mut RawTerminal<Stdout>,
    raw_input: &str,
    cmd: Command,
) -> Result<()> {
    let mut disp_buffer = String::new();

    let preedit = &cmd.response.preedit;
    let mut char_count = 0;

    for (i, segment) in preedit.segments.iter().enumerate() {
        if cmd.response.preedit.focused_caret == i as i32 {
            disp_buffer.push('>');
        }

        if preedit.caret == char_count {
            disp_buffer.push('|');
        }

        for ch in segment.value.chars().collect::<Vec<char>>() {
            disp_buffer.push(ch);
            char_count += 1
        }
    }

    if preedit.caret == char_count {
        disp_buffer.push('|');
    }

    let cands = get_candidate_page(&cmd);
    // let cl = &cmd.response.candidate_list;
    // let cands: Vec<&str> =
    //     cl.candidates.iter().map(|c| c.value.as_str()).collect();

    update_display(stdout, &raw_input, &disp_buffer, &cands)
}

fn draw_footer(stdout: &mut RawTerminal<Stdout>) -> Result<()> {
    let (_, rows) = termion::terminal_size()?;
    let help = vec!["<Esc>: Quit", "<Enter>: Clear"];
    let max_len = help.iter().map(|s| s.chars().count()).max().unwrap_or(0) + 4;

    let formatted: Vec<String> = help
        .into_iter()
        .map(|s| format!("{:>width$}", s, width = max_len))
        .collect();

    write!(stdout, "{}{}", Goto(2, rows - 1), formatted.join(""))?;

    Ok(())
}

pub fn run() -> Result<()> {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut engine = EngineCtrl::new(get_db_filename()?)?;

    blank_display(&mut stdout)?;

    let mut raw_input = String::new();
    let mut disp_buffer = String::new();

    for key in stdin.keys() {
        disp_buffer.clear();
        let key = key?;

        if key == Key::Esc {
            break;
        }

        if let Key::Char(c) = key {
            if c == '\n' {
                raw_input.clear();
                disp_buffer.clear();
                engine.reset()?;
                blank_display(&mut stdout)?;
                continue;
            }

            raw_input.push(c);
        }

        let cmd = engine.send_key(key)?;
        draw_ime(&mut stdout, &raw_input, cmd)?;
    }
    clear(&mut stdout)?;
    Ok(())
}