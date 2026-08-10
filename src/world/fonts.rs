use typst::foundations::Bytes;
use typst::text::{Font, FontBook};

pub(crate) struct LoadedFonts {
    pub book: FontBook,
    pub fonts: Vec<Font>,
}

pub(crate) fn load_fonts() -> Result<LoadedFonts, crate::PdfError> {
    let fonts: Vec<Font> = typst_assets::fonts()
        .flat_map(|data| Font::iter(Bytes::new(data)))
        .collect();

    if fonts.is_empty() {
        return Err(crate::PdfError::FontLoad(
            "typst-assets yielded no fonts".into(),
        ));
    }

    let book = FontBook::from_fonts(&fonts);
    Ok(LoadedFonts { book, fonts })
}
