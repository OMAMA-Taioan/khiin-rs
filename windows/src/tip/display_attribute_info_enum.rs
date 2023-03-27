use std::cell::Cell;

use windows::core::implement;
use windows::core::Error;
use windows::core::Result;
use windows::Win32::Foundation::S_FALSE;
use windows::Win32::UI::TextServices::IEnumTfDisplayAttributeInfo;
use windows::Win32::UI::TextServices::IEnumTfDisplayAttributeInfo_Impl;
use windows::Win32::UI::TextServices::ITfDisplayAttributeInfo;

use crate::tip::display_attribute_info::*;
use crate::reg::guids::IID_DISPLAY_ATTRIBUTE_CONVERTED;
use crate::reg::guids::IID_DISPLAY_ATTRIBUTE_FOCUSED;
use crate::reg::guids::IID_DISPLAY_ATTRIBUTE_INPUT;

#[implement(IEnumTfDisplayAttributeInfo)]
pub struct DisplayAttributeInfoEnum {
    attributes: Vec<DisplayAttributeInfo>,
    current_index: Cell<usize>,
}

impl DisplayAttributeInfoEnum {
    pub fn new() -> Self {
        let mut attributes: Vec<DisplayAttributeInfo> = Vec::new();

        attributes.push(DisplayAttributeInfo {
            description: String::from("Input"),
            guid: IID_DISPLAY_ATTRIBUTE_INPUT,
            attribute: Cell::from(DISPLAY_ATTRIBUTE_INPUT),
        });

        attributes.push(DisplayAttributeInfo {
            description: String::from("Converted"),
            guid: IID_DISPLAY_ATTRIBUTE_CONVERTED,
            attribute: Cell::from(DISPLAY_ATTRIBUTE_CONVERTED),
        });

        attributes.push(DisplayAttributeInfo {
            description: String::from("Focused"),
            guid: IID_DISPLAY_ATTRIBUTE_FOCUSED,
            attribute: Cell::from(DISPLAY_ATTRIBUTE_FOCUSED),
        });

        Self {
            attributes,
            current_index: Cell::from(0),
        }
    }
}

impl IEnumTfDisplayAttributeInfo_Impl for DisplayAttributeInfoEnum {
    fn Clone(&self) -> Result<IEnumTfDisplayAttributeInfo> {
        unsafe {
            DisplayAttributeInfoEnum {
                attributes: self.attributes.clone(),
                current_index: self.current_index.clone(),
            }
            .cast()
        }
    }

    fn Next(
        &self,
        ulcount: u32,
        rginfo: *mut Option<ITfDisplayAttributeInfo>,
        pcfetched: *mut u32,
    ) -> Result<()> {
        let num_attrs = self.attributes.len();

        let mut curr_index = self.current_index.get();
        let mut out_count = 0u32;

        while out_count < ulcount {
            if curr_index >= num_attrs {
                break;
            }

            let out: ITfDisplayAttributeInfo =
                self.attributes[curr_index].clone().into();

            unsafe {
                *rginfo.add(out_count as usize) = Some(out);
            }

            curr_index += 1;
            out_count += 1;
        }

        self.current_index.set(curr_index);

        unsafe {
            *pcfetched = out_count;
        }

        if out_count == ulcount {
            Ok(())
        } else {
            Err(Error::from(S_FALSE))
        }
    }

    fn Reset(&self) -> Result<()> {
        self.current_index.set(0);
        Ok(())
    }

    fn Skip(&self, ulcount: u32) -> Result<()> {
        let count = ulcount as usize;
        let curr_index = self.current_index.get();
        let num_attrs = self.attributes.len();

        let remainder = num_attrs - curr_index - 1;
        if count > remainder {
            self.current_index.set(num_attrs - 1);
            Err(Error::from(S_FALSE))
        } else {
            self.current_index.set(curr_index + count);
            Ok(())
        }
    }
}
