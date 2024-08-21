use crossterm::event::KeyCode as CTKeyCode;
use crossterm::event::KeyEvent as CTKeyEvent;
use crossterm::event::KeyModifiers as CTKeyModifiers;
use khiin_protos::command::KeyEvent as KhiEvent;
use khiin_protos::command::SpecialKey;
use khiin_protos::command::ModifierKey;

pub fn translate_keys(key: CTKeyEvent) -> KhiEvent {
    let mut ret = KhiEvent::new();

    let special_key = match key.code {
        CTKeyCode::Backspace => SpecialKey::SK_BACKSPACE,
        CTKeyCode::Enter => SpecialKey::SK_ENTER,
        CTKeyCode::Left => SpecialKey::SK_LEFT,
        CTKeyCode::Right => SpecialKey::SK_RIGHT,
        CTKeyCode::Up => SpecialKey::SK_UP,
        CTKeyCode::Down => SpecialKey::SK_DOWN,
        CTKeyCode::Home => SpecialKey::SK_HOME,
        CTKeyCode::End => SpecialKey::SK_END,
        CTKeyCode::PageUp => SpecialKey::SK_PGUP,
        CTKeyCode::PageDown => SpecialKey::SK_PGDN,
        CTKeyCode::Tab => SpecialKey::SK_TAB,
        CTKeyCode::Delete => SpecialKey::SK_DEL,
        _ => SpecialKey::SK_NONE,
    };

    let char = match key.code {
        CTKeyCode::Char(c) => c as i32,
        _ => 0,
    };

    ret.key_code = char;
    ret.special_key = if char == ' ' as i32 {
        SpecialKey::SK_SPACE.into()
    } else {
        special_key.into()
    };

    if key.modifiers.contains(CTKeyModifiers::SHIFT) {
        ret.modifier_keys.push(ModifierKey::MODK_SHIFT.into());
    }

    ret
}
