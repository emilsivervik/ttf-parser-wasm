use ttf_parser::maxp;
use wasm_bindgen::prelude::wasm_bindgen;

/// A [Maximum Profile Table](https://docs.microsoft.com/en-us/typography/opentype/spec/maxp).
#[wasm_bindgen(js_name = "MAXPTable")]
#[derive(Clone, Copy)]
pub struct Table {
    /// The total number of glyphs in the face.
    #[wasm_bindgen(js_name = "numberOfGlyphs")]
    pub number_of_glyphs: u16,
}

impl Table {
    pub fn new(table: maxp::Table) -> Self {
        Self {
            number_of_glyphs: table.number_of_glyphs.get(),
        }
    }
}
