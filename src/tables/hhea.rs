use ttf_parser::hhea;
use wasm_bindgen::prelude::wasm_bindgen;

/// A [Horizontal Header Table](https://docs.microsoft.com/en-us/typography/opentype/spec/hhea).
#[wasm_bindgen(js_name = "HHEATable")]
#[derive(Clone, Copy)]
pub struct Table {
    /// Face ascender.
    pub ascender: i16,
    /// Face descender.
    pub descender: i16,
    /// Face line gap.
    #[wasm_bindgen(js_name = "lineGap")]
    pub line_gap: i16,
    /// Number of metrics in the `hmtx` table.
    #[wasm_bindgen(js_name = "numberOfMetrics")]
    pub number_of_metrics: u16,
}

impl Table {
    pub fn new(table: hhea::Table) -> Self {
        Self {
            ascender: table.ascender,
            descender: table.descender,
            line_gap: table.line_gap,
            number_of_metrics: table.number_of_metrics,
        }
    }
}
