use std::io::stdin;
use std::io::stdout;
use std::io::Stdout;
use std::io::Write;

use anyhow::Result;
use khiin::Engine;
use khiin_protos::command::Command;
use khiin_protos::command::CommandType;
use khiin_protos::command::KeyEvent;
use khiin_protos::command::Request;
use khiin_protos::command::SpecialKey;
use protobuf::Message;
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

fn translate_keys(key: Key) -> KeyEvent {
    let mut ret = KeyEvent::new();

    let special_key = match key {
        Key::Backspace => SpecialKey::SK_BACKSPACE,
        Key::Left => SpecialKey::SK_LEFT,
        Key::Right => SpecialKey::SK_RIGHT,
        Key::Up => SpecialKey::SK_UP,
        Key::Down => SpecialKey::SK_DOWN,
        Key::Home => SpecialKey::SK_HOME,
        Key::End => SpecialKey::SK_END,
        Key::PageUp => SpecialKey::SK_PGUP,
        Key::PageDown => SpecialKey::SK_PGDN,
        Key::BackTab => SpecialKey::SK_TAB,
        Key::Delete => SpecialKey::SK_DEL,
        Key::Esc => SpecialKey::SK_ESC,
        _ => SpecialKey::SK_NONE,
    };

    let char = match key {
        Key::Char(c) => c as i32,
        _ => 0,
    };

    ret.key_code = char;
    ret.special_key = special_key.into();

    ret
}

fn build_command(key: Key) -> Command {
    let key_event = translate_keys(key);

    let mut req = Request::new();
    req.key_event = Some(key_event).into();
    req.type_ = CommandType::CMD_SEND_KEY.into();

    let mut cmd = Command::new();
    cmd.request = Some(req).into();
    cmd
}

fn send_key(engine: &mut Engine, key: Key) -> Result<Command> {
    let cmd = build_command(key);
    let bytes = cmd.write_to_bytes()?;
    let bytes = engine.send_command_bytes(&bytes)?;
    let cmd = Command::parse_from_bytes(&bytes)?;
    Ok(cmd)
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

fn update_display(
    stdout: &mut RawTerminal<Stdout>,
    raw: &str,
    display: &str,
    cands: &Vec<&str>,
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

    for (i, cand) in cands.iter().take(9).enumerate() {
        write!(stdout, "{}{}. {}", Goto(15, 8 + i as u16), i + 1, cand)?;
    }

    stdout.flush()?;
    Ok(())
}

fn main() -> Result<()> {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut db_path = std::env::current_exe()?;
    db_path.set_file_name("khiin.db");

    let mut engine = Engine::new(db_path.to_str().unwrap()).unwrap();

    clear(&mut stdout)?;
    update_display(&mut stdout, "", "", &Vec::new())?;

    let mut raw_input = String::new();
    let mut disp_buffer = String::new();

    for key in stdin.keys() {
        disp_buffer.clear();
        let key = key?;

        if key == Key::Esc {
            break;
        }

        if let Key::Char(c) = key {
            raw_input.push(c);
        }

        let cmd = send_key(&mut engine, key)?;

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

        let cands: Vec<&str> = cmd
            .response
            .candidate_list
            .candidates
            .iter()
            .map(|c| c.value.as_str())
            .collect();

        update_display(&mut stdout, &raw_input, &disp_buffer, &cands)?;
    }

    Ok(())
}
