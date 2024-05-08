use log::debug as d;

use windows::core::Interface;
use windows::core::Result;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Foundation::RECT;
use windows::Win32::UI::Input::KeyboardAndMouse::GetFocus;
use windows::Win32::UI::TextServices::IEnumITfCompositionView;
use windows::Win32::UI::TextServices::ITfCompositionView;
use windows::Win32::UI::TextServices::ITfContext;
use windows::Win32::UI::TextServices::ITfContextComposition;
use windows::Win32::UI::TextServices::ITfContextView;
use windows::Win32::UI::TextServices::ITfRange;
use windows::Win32::UI::TextServices::TF_ANCHOR_START;
use windows::Win32::UI::TextServices::TF_SELECTION;
use windows::Win32::UI::TextServices::TF_ST_CORRECTION;
use windows::Win32::UI::WindowsAndMessaging::GetWindowRect;

use crate::fail;
use crate::geometry::Rect;
use crate::tip::TextService;
use crate::utils::WinString;

// Getting the position of composition text on screen with
// ITfContextView::GetTextExt seems rather unreliable in some applications, so
// we include some fallbacks. If everything works as expected, we will get the
// range from the ITfContext directly (via the ITfCompositionView). If not, we
// try the default selection. Finally, if both fail, we put the window in the
// upper left corner of the parent window, which is not ideal but still better
// than falling back to 0,0 at the upper left corner of the entire screen.
pub fn text_position(
    ec: u32,
    context: ITfContext,
    caret: i32,
) -> Result<Rect<i32>> {
    unsafe {
        let context_view = context.GetActiveView()?;
        let range = composition_range(ec, context.clone())?;
        let mut shifted = 0i32;

        range.Collapse(ec, TF_ANCHOR_START)?;
        range.ShiftEnd(ec, caret, &mut shifted, std::ptr::null())?;
        range.ShiftStart(ec, caret, &mut shifted, std::ptr::null())?;

        let mut rect = RECT::default();
        let mut clipped = BOOL::default();
        context_view.GetTextExt(ec, &range, &mut rect, &mut clipped)?;

        if !empty_rect(&rect) {
            d!("text position from composition range: {:?}", rect);
            return Ok((&rect).into());
        }

        let text = " ".to_utf16_nul();
        let range = default_selection_range(ec, context)?;
        range.SetText(ec, TF_ST_CORRECTION, &text)?;
        range.Collapse(ec, TF_ANCHOR_START)?;
        context_view.GetTextExt(ec, &range, &mut rect, &mut clipped)?;

        if !empty_rect(&rect) {
            d!("text position from default selection range: {:?}", rect);
            return Ok((&rect).into());
        }

        let rect = parent_window_origin(context_view)?;
        d!("text position parent window origin: {:?}", rect);
        Ok((&rect).into())
    }
}

unsafe fn composition_range(ec: u32, context: ITfContext) -> Result<ITfRange> {
    let composition_view = composition_view(ec, context)?;
    let range = composition_view.GetRange()?;
    let clone = range.Clone()?;
    Ok(clone)
}

unsafe fn composition_view(
    ec: u32,
    context: ITfContext,
) -> Result<ITfCompositionView> {
    let cc: ITfContextComposition = context.cast()?;
    let enum_comp: IEnumITfCompositionView = cc.FindComposition(ec, None)?;
    let mut vec: Vec<Option<ITfCompositionView>> = Vec::new();
    let mut idx: usize = 0;

    loop {
        vec.push(None);
        let mut fetched = 0u32;
        enum_comp.Next(vec.as_mut_slice(), &mut fetched)?;

        if fetched != 1 {
            return Err(fail!());
        }

        let view = vec[idx].clone().ok_or(fail!())?;
        let clsid = view.GetOwnerClsid()?;

        if clsid != TextService::IID {
            idx += 1;
            continue;
        }

        return Ok(view);
    }
}

unsafe fn default_selection_range(
    ec: u32,
    context: ITfContext,
) -> Result<ITfRange> {
    let mut vec: Vec<TF_SELECTION> = Vec::new();
    let mut selection = TF_SELECTION::default();
    let mut fetched = 0u32;
    // TODO https://github.com/microsoft/windows-rs/issues/2429
    context.GetSelection(ec, u32::MAX, &mut vec, &mut fetched)?;
    vec.get(0).ok_or(fail!())?.range.as_ref().unwrap().Clone()
}

unsafe fn parent_window_origin(view: ITfContextView) -> Result<RECT> {
    let mut rect = RECT::default();
    let handle = match view.GetWnd() {
        Ok(handle) => handle,
        Err(_) => GetFocus(),
    };
    let found = GetWindowRect(handle, &mut rect);

    Ok(if found.is_ok() {
        RECT {
            left: rect.left,
            top: rect.top,
            right: rect.left + 1,
            bottom: rect.top + 1,
        }
    } else {
        rect
    })
}

fn empty_rect(rect: &RECT) -> bool {
    rect.left == 0 && rect.top == 0 && rect.right == 0 && rect.bottom == 0
}
