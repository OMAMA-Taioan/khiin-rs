#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[cfg(windows)]
mod app {
    mod locales;
    mod macros;
    mod pages;
    mod propsheet;
    mod resource;
    mod windowsx;
    mod winuser;
    
    use std::rc::Rc;
    
    use pages::AboutPage;
    use pages::InputPage;
    use pages::StylePage;
    use pages::UserDictPage;
    use propsheet::PropSheet;
    use resource::*;
    use windows::core::Result;
    use windows::core::PCWSTR;
    use windows::Win32::System::LibraryLoader::GetModuleHandleW;
    
    pub(crate) fn run() -> Result<isize> {
        let module = unsafe { GetModuleHandleW(PCWSTR::null())? };
        let mut propsheet = PropSheet::new(module);
    
        propsheet.add_page(IDD_APPEARANCETAB, Rc::new(StylePage::default()));
        propsheet.add_page(IDD_INPUTTAB, Rc::new(InputPage::default()));
        propsheet.add_page(IDD_DICTIONARYTAB, Rc::new(UserDictPage::default()));
        propsheet.add_page(IDD_ABOUTTAB, Rc::new(AboutPage::default()));
    
        let pid = propsheet.run();
        Ok(pid)
    }
}

#[cfg(windows)]
fn main() {
    app::run().unwrap();
}

#[cfg(not(windows))]
fn main() {
    println!("Not implemented for operating systems other than Windows")
}
