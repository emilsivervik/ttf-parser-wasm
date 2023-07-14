use super::Rect;
use ttf_parser::head;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum IndexToLocationFormat {
    Short = "Short",
    Long = "Long",
}

impl From<head::IndexToLocationFormat> for IndexToLocationFormat {
    fn from(index_to_location_format: head::IndexToLocationFormat) -> IndexToLocationFormat {
        match index_to_location_format {
            head::IndexToLocationFormat::Long => IndexToLocationFormat::Long,
            head::IndexToLocationFormat::Short => IndexToLocationFormat::Short,
        }
    }
}

/// A [Font Header Table](https://docs.microsoft.com/en-us/typography/opentype/spec/head).
#[wasm_bindgen(js_name = "HEADTable")]
#[derive(Clone, Copy)]
pub struct Table {
    /// Units per EM.
    #[wasm_bindgen(js_name = "unitsPerEm")]
    pub units_per_em: u16,
    /// A bounding box that large enough to enclose any glyph from the face.
    #[wasm_bindgen(js_name = "globalBbox")]
    pub global_bbox: Rect,
    /// An index format used by the [Index to Location Table](
    /// https://docs.microsoft.com/en-us/typography/opentype/spec/loca).
    #[wasm_bindgen(js_name = "indexToLocationFormat")]
    pub index_to_location_format: IndexToLocationFormat,
}

impl Table {
    pub fn new(table: head::Table) -> Self {
        Self {
            index_to_location_format: table.index_to_location_format.into(),
            units_per_em: table.units_per_em,
            global_bbox: table.global_bbox.into(),
        }
    }
}
