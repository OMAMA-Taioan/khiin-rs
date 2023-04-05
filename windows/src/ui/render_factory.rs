// use once_cell::sync::Lazy;

use windows::core::Result;
use windows::core::HSTRING;
use windows::core::PCWSTR;
use windows::Win32::Graphics::Direct2D::Common::D2D1_ALPHA_MODE_IGNORE;
use windows::Win32::Graphics::Direct2D::Common::D2D1_PIXEL_FORMAT;
use windows::Win32::Graphics::Direct2D::D2D1CreateFactory;
use windows::Win32::Graphics::Direct2D::ID2D1Bitmap;
use windows::Win32::Graphics::Direct2D::ID2D1DCRenderTarget;
use windows::Win32::Graphics::Direct2D::ID2D1Factory1;
use windows::Win32::Graphics::Direct2D::D2D1_FACTORY_TYPE_SINGLE_THREADED;
use windows::Win32::Graphics::Direct2D::D2D1_FEATURE_LEVEL_DEFAULT;
use windows::Win32::Graphics::Direct2D::D2D1_RENDER_TARGET_PROPERTIES;
use windows::Win32::Graphics::Direct2D::D2D1_RENDER_TARGET_TYPE_DEFAULT;
use windows::Win32::Graphics::Direct2D::D2D1_RENDER_TARGET_USAGE_NONE;
use windows::Win32::Graphics::DirectWrite::DWriteCreateFactory;
use windows::Win32::Graphics::DirectWrite::IDWriteFactory3;
use windows::Win32::Graphics::DirectWrite::IDWriteTextFormat;
use windows::Win32::Graphics::DirectWrite::IDWriteTextLayout;
use windows::Win32::Graphics::DirectWrite::DWRITE_FACTORY_TYPE_SHARED;
use windows::Win32::Graphics::DirectWrite::DWRITE_FONT_STRETCH_NORMAL;
use windows::Win32::Graphics::DirectWrite::DWRITE_FONT_STYLE_NORMAL;
use windows::Win32::Graphics::DirectWrite::DWRITE_FONT_WEIGHT_NORMAL;
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM;
use windows::Win32::Graphics::Imaging::CLSID_WICImagingFactory;
use windows::Win32::Graphics::Imaging::GUID_WICPixelFormat32bppPBGRA;
use windows::Win32::Graphics::Imaging::IWICImagingFactory;
use windows::Win32::Graphics::Imaging::WICBitmapDitherTypeNone;
use windows::Win32::Graphics::Imaging::WICBitmapPaletteTypeMedianCut;
use windows::Win32::System::Com::CoCreateInstance;
use windows::Win32::System::Com::CLSCTX_INPROC_SERVER;
use windows::Win32::UI::WindowsAndMessaging::HICON;

use crate::utils::ToPcwstr;
use crate::utils::WinString;
// use crate::utils::co_create_inproc;

// static D2D1_FACTORY: Lazy<ID2D1Factory1> = Lazy::new(|| unsafe {
//     D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None).unwrap()
// });

// static DWRITE_FACTORY: Lazy<IDWriteFactory3> = Lazy::new(|| unsafe {
//     DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED).unwrap()
// });

// IWICImagingFactory does not implement Sync, otherwise we could just
// use these statics...

// static WIC_FACTORY: Lazy<IWICImagingFactory> = Lazy::new(|| unsafe {
//     let wic = co_create_inproc(&CLSID_WICImagingFactory).unwrap();
//     wic
// });

pub struct RenderFactory {
    d2d1_factory: ID2D1Factory1,
    dwrite_factory: IDWriteFactory3,
    wic_factory: IWICImagingFactory,
}

impl RenderFactory {
    pub fn new() -> Result<Self> {
        unsafe {
            let d2d1_factory: ID2D1Factory1 =
                D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None)?;

            let dwrite_factory: IDWriteFactory3 =
                DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)?;

            let wic_factory: IWICImagingFactory = CoCreateInstance(
                &CLSID_WICImagingFactory,
                None,
                CLSCTX_INPROC_SERVER,
            )?;

            Ok(Self {
                d2d1_factory,
                dwrite_factory,
                wic_factory,
            })
        }
    }

    pub fn create_dc_render_target(&self) -> Result<ID2D1DCRenderTarget> {
        let props = D2D1_RENDER_TARGET_PROPERTIES {
            r#type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
            pixelFormat: D2D1_PIXEL_FORMAT {
                format: DXGI_FORMAT_B8G8R8A8_UNORM,
                alphaMode: D2D1_ALPHA_MODE_IGNORE,
            },
            dpiX: 0.0,
            dpiY: 0.0,
            usage: D2D1_RENDER_TARGET_USAGE_NONE,
            minLevel: D2D1_FEATURE_LEVEL_DEFAULT,
        };
        unsafe { self.d2d1_factory.CreateDCRenderTarget(&props) }
    }

    pub fn create_text_format(
        &self,
        fontname: &str,
        fontsize: f32,
    ) -> Result<IDWriteTextFormat> {
        unsafe {
            let fontname = PCWSTR(HSTRING::from(fontname).as_ptr());
            let locale = "en-us".to_pcwstr();
            // let mut collection: Option<IDWriteFontCollection> = None;
            // self.dwrite_factory
            //     .GetSystemFontCollection(&mut collection, BOOL::from(false))?;
            // let collection = collection.unwrap();

            self.dwrite_factory.CreateTextFormat(
                fontname,
                None, // collection
                DWRITE_FONT_WEIGHT_NORMAL,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                fontsize,
                *locale,
            )
        }
    }

    pub fn create_text_layout(
        &self,
        value: String,
        format: IDWriteTextFormat,
        max_width: f32,
        max_height: f32,
    ) -> Result<IDWriteTextLayout> {
        let value = value.to_utf16();

        unsafe {
            self.dwrite_factory
                .CreateTextLayout(&value, &format, max_width, max_height)
        }

        // add fallbacks using IDWriteTextLayout2?
    }

    pub fn create_bitmap(
        &self,
        target: ID2D1DCRenderTarget,
        hicon: HICON,
    ) -> Result<ID2D1Bitmap> {
        unsafe {
            let wic_bmp = self.wic_factory.CreateBitmapFromHICON(hicon)?;
            let converter = self.wic_factory.CreateFormatConverter()?;
            converter.Initialize(
                &wic_bmp,
                &GUID_WICPixelFormat32bppPBGRA,
                WICBitmapDitherTypeNone,
                None,
                0.0,
                WICBitmapPaletteTypeMedianCut,
            )?;
            target.CreateBitmapFromWicBitmap(&converter, None)
        }
    }
}
