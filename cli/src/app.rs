use std::io::Stdout;
use std::io::Write;

use anyhow::Result;
use crossterm::cursor::MoveTo;
use crossterm::cursor::SetCursorStyle;
use crossterm::cursor::Show;
use crossterm::event::read;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyEventKind;
use crossterm::execute;
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::size;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;
use crossterm::terminal::EnterAlternateScreen;
use khiin_protos::command::Command;
use khiin_protos::command::SegmentStatus;
use khiin_protos::config::AppInputMode;
use unicode_width::UnicodeWidthStr;

use crate::engine_ctrl::EngineCtrl;

fn get_db_filename() -> Result<String> {
    let mut db_path = std::env::current_exe()?;
    db_path.set_file_name("khiin.db");
    Ok(db_path.to_str().unwrap().to_string())
}

fn clear(stdout: &mut Stdout) -> Result<()> {
    queue!(stdout, Clear(ClearType::All), MoveTo(1, 1))?;
    stdout.flush()?;
    Ok(())
}

fn blank_display(stdout: &mut Stdout, mode: &AppInputMode) -> Result<()> {
    let input_mode = match mode {
        AppInputMode::CONTINUOUS => "Auto",
        AppInputMode::CLASSIC => "Classic",
        AppInputMode::MANUAL => "Manual",
    };
    update_display(stdout, &input_mode, "", "", "", 0, "", &Vec::new())?;
    Ok(())
}

fn update_display(
    stdout: &mut Stdout,
    mode: &str,
    raw: &str,
    display: &str,
    committed: &str,
    caret: usize,
    attrs: &str,
    cands: &Vec<String>,
) -> Result<()> {
    clear(stdout)?;
    execute!(
        stdout,
        MoveTo(2, 2),
        Print("Khíín Phah Jī Hoat"),
        MoveTo(2, 4),
        Print(format!("Input mode:  {}", mode)),
        MoveTo(2, 6),
        Print(format!("Raw input:  {}", raw)),
        MoveTo(2, 8),
        Print(format!("Committed:  {}", committed)),
        MoveTo(2, 10),
        Print(format!("User sees:  {}", display)),
        MoveTo(14, 11),
        Print(format!("{}", attrs)),
        MoveTo(2, 12),
        Print(format!("Candidates:")),
    )?;

    for (i, cand) in cands.iter().enumerate() {
        execute!(
            stdout,
            MoveTo(15, 12 + i as u16),
            Print(format!("{}", cand))
        )?;
    }

    draw_footer(stdout)?;
    execute!(
        stdout,
        MoveTo(14 + caret as u16, 10),
        Show,
        SetCursorStyle::BlinkingBar
    )?;
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
    stdout: &mut Stdout,
    raw_input: &str,
    done_buffer: &mut String,
    cmd: Command,
    mode: &AppInputMode,
) -> Result<()> {
    let mut disp_buffer = String::new();
    let mut attr_buffer = String::new();

    let preedit = &cmd.response.preedit;
    let mut char_count = 0;
    let mut caret = 0;

    for segment in preedit.segments.iter() {
        let mut disp_seg = String::new();

        if preedit.caret == char_count {
            caret = disp_buffer.width() + disp_seg.width();
        }

        for ch in segment.value.chars().collect::<Vec<char>>() {
            disp_seg.push(ch);
            char_count += 1
        }

        let attr = match segment.status.enum_value_or_default() {
            SegmentStatus::SS_UNMARKED => ' ',
            SegmentStatus::SS_COMPOSING => '┄',
            SegmentStatus::SS_CONVERTED => '─',
            SegmentStatus::SS_FOCUSED => '━',
        };

        let seg_width = disp_seg.width();
        let seg_attr = attr.to_string().repeat(seg_width);
        disp_buffer.push_str(&disp_seg);
        attr_buffer.push_str(&seg_attr);
    }

    if preedit.caret == char_count {
        caret = disp_buffer.width();
    }

    let cands = get_candidate_page(&cmd);
    let input_mode = match mode {
        AppInputMode::CONTINUOUS => "Auto",
        AppInputMode::CLASSIC => "Classic",
        AppInputMode::MANUAL => "Manual",
    };

    if cmd.response.committed {
        done_buffer.push_str(&disp_buffer);
        disp_buffer.clear();
        caret = 0;
    }

    update_display(
        stdout,
        &input_mode,
        &raw_input,
        &disp_buffer,
        done_buffer,
        caret,
        &attr_buffer,
        &cands,
    )
}

fn draw_footer(stdout: &mut Stdout) -> Result<()> {
    let (_, rows) = size()?;

    let help = vec!["<Esc>: Quit", "<Enter>: Clear", "<Tab>: Switch mode"];

    let max_len = help.iter().map(|s| s.chars().count()).max().unwrap_or(0) + 4;

    let formatted: Vec<String> = help
        .into_iter()
        .map(|s| format!("{:>width$}", s, width = max_len))
        .collect();

    execute!(
        stdout,
        MoveTo(2, rows - 1),
        Print(format!("{}", formatted.join("")))
    )?;

    Ok(())
}

fn read_key() -> Result<KeyEvent> {
    loop {
        if let Event::Key(event) = read()? {
            return Ok(event);
        }
    }
}

pub fn run(stdout: &mut Stdout) -> Result<()> {
    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut engine = EngineCtrl::new(get_db_filename()?)?;
    let mut mode: AppInputMode = AppInputMode::CLASSIC;

    engine.send_set_config_command(&mode, true)?;
    blank_display(stdout, &mode)?;

    let mut raw_input = String::new();
    let mut done_buffer = String::new();

    loop {
        let key = read_key()?;

        if key.kind != KeyEventKind::Press {
            continue;
        }

        if key.code == KeyCode::Esc {
            break;
        }

        if key.code == KeyCode::Tab {
            if mode == AppInputMode::CONTINUOUS {
                mode = AppInputMode::CLASSIC;
            } else if mode == AppInputMode::CLASSIC {
                mode = AppInputMode::MANUAL;
            } else {
                mode = AppInputMode::CONTINUOUS;
            }
            raw_input.clear();
            done_buffer.clear();
            let cmd = engine.send_switch_mode_command(&mode)?;
            draw_ime(stdout, &raw_input, &mut done_buffer, cmd, &mode)?;
            continue;
        }

        match key.code {
            KeyCode::Enter => {
                raw_input.clear();
                done_buffer.clear();
                engine.reset()?;
                blank_display(stdout, &mode)?;
                continue;
            },
            KeyCode::Backspace => {
                if !raw_input.is_empty() {
                    raw_input.push_str("<Back>")
                }
            },
            KeyCode::Char(c) => {
                if c != ' ' {
                    raw_input.push(c);
                }
            },
            _ => {},
        }

        let cmd = engine.send_key(key)?;
        if cmd.response.committed {
            raw_input.clear();
        }
        draw_ime(stdout, &raw_input, &mut done_buffer, cmd, &mode)?;
    }

    clear(stdout)?;

    disable_raw_mode().map_err(|e| anyhow::Error::from(e))
}
