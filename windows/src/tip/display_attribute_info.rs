use std::cell::Cell;

use windows::core::implement;
use windows::core::Result;
use windows::core::BSTR;
use windows::core::GUID;
use windows::Win32::Foundation::FALSE;
use windows::Win32::UI::TextServices::ITfDisplayAttributeInfo;
use windows::Win32::UI::TextServices::ITfDisplayAttributeInfo_Impl;
use windows::Win32::UI::TextServices::TF_ATTR_CONVERTED;
use windows::Win32::UI::TextServices::TF_ATTR_INPUT;
use windows::Win32::UI::TextServices::TF_ATTR_TARGET_CONVERTED;
use windows::Win32::UI::TextServices::TF_CT_NONE;
use windows::Win32::UI::TextServices::TF_DA_COLOR;
use windows::Win32::UI::TextServices::TF_DA_COLOR_0;
use windows::Win32::UI::TextServices::TF_DISPLAYATTRIBUTE;
use windows::Win32::UI::TextServices::TF_LS_SOLID;
use windows::Win32::UI::TextServices::TF_LS_SQUIGGLE;

pub const DISPLAY_ATTRIBUTE_INPUT: TF_DISPLAYATTRIBUTE = TF_DISPLAYATTRIBUTE {
    crText: TF_DA_COLOR {
        r#type: TF_CT_NONE,
        Anonymous: TF_DA_COLOR_0 { nIndex: 0 },
    },
    crBk: TF_DA_COLOR {
        r#type: TF_CT_NONE,
        Anonymous: TF_DA_COLOR_0 { nIndex: 0 },
    },
    lsStyle: TF_LS_SQUIGGLE,
    fBoldLine: FALSE,
    crLine: TF_DA_COLOR {
        r#type: TF_CT_NONE,
        Anonymous: TF_DA_COLOR_0 { nIndex: 0 },
    },
    bAttr: TF_ATTR_INPUT,
};

pub const DISPLAY_ATTRIBUTE_CONVERTED: TF_DISPLAYATTRIBUTE =
    TF_DISPLAYATTRIBUTE {
        crText: TF_DA_COLOR {
            r#type: TF_CT_NONE,
            Anonymous: TF_DA_COLOR_0 { nIndex: 0 },
        },
        crBk: TF_DA_COLOR {
            r#type: TF_CT_NONE,
            Anonymous: TF_DA_COLOR_0 { nIndex: 0 },
        },
        lsStyle: TF_LS_SOLID,
        fBoldLine: FALSE,
        crLine: TF_DA_COLOR {
            r#type: TF_CT_NONE,
            Anonymous: TF_DA_COLOR_0 { nIndex: 0 },
        },
        bAttr: TF_ATTR_CONVERTED,
    };

pub const DISPLAY_ATTRIBUTE_FOCUSED: TF_DISPLAYATTRIBUTE =
    TF_DISPLAYATTRIBUTE {
        crText: TF_DA_COLOR {
            r#type: TF_CT_NONE,
            Anonymous: TF_DA_COLOR_0 { nIndex: 0 },
        },
        crBk: TF_DA_COLOR {
            r#type: TF_CT_NONE,
            Anonymous: TF_DA_COLOR_0 { nIndex: 0 },
        },
        lsStyle: TF_LS_SOLID,
        fBoldLine: FALSE,
        crLine: TF_DA_COLOR {
            r#type: TF_CT_NONE,
            Anonymous: TF_DA_COLOR_0 { nIndex: 0 },
        },
        bAttr: TF_ATTR_TARGET_CONVERTED,
    };

#[implement(ITfDisplayAttributeInfo)]
#[derive(Clone)]
pub struct DisplayAttributeInfo {
    description: String,
    guid: GUID,
    attribute: Cell<TF_DISPLAYATTRIBUTE>,
    attribute_backup: TF_DISPLAYATTRIBUTE,
}

impl DisplayAttributeInfo {
    pub fn new(
        description: String,
        guid: GUID,
        attribute: TF_DISPLAYATTRIBUTE,
    ) -> ITfDisplayAttributeInfo {
        DisplayAttributeInfo {
            description,
            guid,
            attribute: Cell::new(attribute),
            attribute_backup: attribute,
        }
        .into()
    }
}

impl ITfDisplayAttributeInfo_Impl for DisplayAttributeInfo {
    fn GetGUID(&self) -> Result<GUID> {
        Ok(self.guid)
    }

    fn GetDescription(&self) -> Result<BSTR> {
        Ok(BSTR::from(self.description.clone()))
    }

    fn GetAttributeInfo(&self, pda: *mut TF_DISPLAYATTRIBUTE) -> Result<()> {
        unsafe {
            *pda = self.attribute.get();
        }

        Ok(())
    }

    fn SetAttributeInfo(&self, pda: *const TF_DISPLAYATTRIBUTE) -> Result<()> {
        unsafe {
            self.attribute.set(*pda);
        }

        Ok(())
    }

    fn Reset(&self) -> Result<()> {
        self.attribute.set(self.attribute_backup);
        Ok(())
    }
}
