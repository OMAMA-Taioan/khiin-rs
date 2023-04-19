use khiin_protos::command::KeyEvent;
use khiin_protos::command::SpecialKey;
use termion::event::Key;

pub fn translate_keys(key: Key) -> KeyEvent {
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
