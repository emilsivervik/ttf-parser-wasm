use tables::os2::{Permissions, ScriptMetrics};
use ttf_parser::Face;
use wasm_bindgen::prelude::*;

mod tables;

pub use tables::{Tables, TablesEnum};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(inspectable)]
pub struct TTFParser {
    tables: Tables,

    /// Checks that face is marked as *Bold*.
    ///
    /// Returns `false` when OS/2 table is not present.
    #[wasm_bindgen(js_name = "isBold", readonly)]
    pub is_bold: bool,

    /// Checks that face is marked as *Italic*.
    ///
    /// Returns `false` when OS/2 table is not present.
    #[wasm_bindgen(js_name = "isItalic", readonly)]
    pub is_italic: bool,

    /// Checks that face is marked as *Monospaced*.
    ///
    /// Returns `false` when `post` table is not present.
    #[wasm_bindgen(js_name = "isMonospaced", readonly)]
    pub is_monospaced: bool,

    /// Checks that face is marked as *Oblique*.
    ///
    /// Returns `false` when OS/2 table is not present or when its version is < 4.
    #[wasm_bindgen(js_name = "isOblique", readonly)]
    pub is_oblique: bool,

    /// Checks that face is marked as *Regular*.
    ///
    /// Returns `false` when OS/2 table is not present.
    #[wasm_bindgen(js_name = "isRegular", readonly)]
    pub is_regular: bool,

    /// Checks that face is variable.
    ///
    /// Simply checks the presence of a `fvar` table.
    #[wasm_bindgen(js_name = "isVariable", readonly)]
    pub is_variable: bool,

    // Read https://github.com/freetype/freetype/blob/49270c17011491227ec7bd3fb73ede4f674aa065/src/sfnt/sfobjs.c#L1279
    // to learn more about the logic behind the following property.
    /// Returns a horizontal face ascender.
    ///
    /// This property is affected by variation axes.
    #[wasm_bindgen(js_name = "ascender", readonly)]
    pub ascender: i16,

    /// Returns a horizontal face descender.
    ///
    /// This property is affected by variation axes.
    #[wasm_bindgen(js_name = "descender", readonly)]
    pub descender: i16,

    /// Returns face's height.
    ///
    /// This property is affected by variation axes.
    #[wasm_bindgen(js_name = "height", readonly)]
    pub height: i16,

    /// Returns a horizontal face line gap.
    ///
    /// This property is affected by variation axes.
    #[wasm_bindgen(js_name = "lineGap", readonly)]
    pub line_gap: i16,

    /// Returns a horizontal typographic face ascender.
    ///
    /// Prefer `ascender` unless you explicitly want this. This is a more
    /// low-level alternative.
    ///
    /// This property is affected by variation axes.
    ///
    /// Returns `None` when OS/2 table is not present.
    #[wasm_bindgen(js_name = "typographicAscender", readonly)]
    pub typographic_ascender: Option<i16>,

    /// Returns a horizontal typographic face ascender.
    ///
    /// Prefer `ascender` unless you explicitly want this. This is a more
    /// low-level alternative.
    ///
    /// This property is affected by variation axes.
    ///
    /// Returns `None` when OS/2 table is not present.
    #[wasm_bindgen(js_name = "typographicDescender", readonly)]
    pub typographic_descender: Option<i16>,

    /// Returns a horizontal typographic face line gap.
    ///
    /// Prefer `line_gap` unless you explicitly want this. This is a more
    /// low-level alternative.
    ///
    /// This property is affected by variation axes.
    ///
    /// Returns `None` when OS/2 table is not present.
    #[wasm_bindgen(js_name = "typographicLineGap", readonly)]
    pub typographic_line_gap: Option<i16>,

    /// Returns a vertical face ascender.
    ///
    /// This property is affected by variation axes.
    #[wasm_bindgen(js_name = "verticalAscender", readonly)]
    pub vertical_ascender: Option<i16>,

    /// Returns a vertical face descender.
    ///
    /// This property is affected by variation axes.
    #[wasm_bindgen(js_name = "verticalDescender", readonly)]
    pub vertical_descender: Option<i16>,

    /// Returns a vertical face height.
    ///
    /// This method is affected by variation axes.
    #[wasm_bindgen(js_name = "verticalHeight", readonly)]
    pub vertical_height: Option<i16>,

    /// Returns a vertical face line gap.
    ///
    /// This property is affected by variation axes.
    #[wasm_bindgen(js_name = "verticalLineGap", readonly)]
    pub vertical_line_gap: Option<i16>,

    /// Returns face's units per EM.
    ///
    /// Guarantee to be in a 16..=16384 range.
    #[wasm_bindgen(js_name = "unitsPerEm", readonly)]
    pub units_per_em: u16,

    /// Returns face's x height.
    ///
    /// This property is affected by variation axes.
    ///
    /// Returns `undefined` when OS/2 table is not present or when its version is < 2.
    #[wasm_bindgen(js_name = "xHeight", readonly)]
    pub x_height: Option<i16>,

    /// Returns face's capital height.
    ///
    /// This property is affected by variation axes.
    ///
    /// Returns `undefined` when OS/2 table is not present or when its version is < 2.
    #[wasm_bindgen(js_name = "capitalHeight", readonly)]
    pub capital_height: Option<i16>,

    /// Returns face's italic angle.
    ///
    /// Returns `undefined` when `post` table is not present.
    #[wasm_bindgen(js_name = "italicAngle", readonly)]
    pub italic_angle: Option<f32>,

    /// Returns face permissions.
    pub permissions: Option<Permissions>,

    /// Checks if the face subsetting is allowed.
    #[wasm_bindgen(js_name = "isSubsettingAllowed", readonly)]
    pub is_subsetting_allowed: bool,

    /// Checks if the face bitmaps embedding is allowed.
    #[wasm_bindgen(js_name = "isBitmapEmbeddingAllowed", readonly)]
    pub is_bitmap_embedding_allowed: bool,

    /// Returns a total number of glyphs in the face.
    ///
    /// Never zero.
    #[wasm_bindgen(js_name = "numberOfGlyphs", readonly)]
    pub number_of_glyphs: u16,

    /// Returns face's superscript metrics.
    ///
    /// This property is affected by variation axes.
    ///
    /// Returns `undefined` when OS/2 table is not present.
    #[wasm_bindgen(js_name = "superScriptMetrics", readonly)]
    pub superscript_metrics: Option<ScriptMetrics>,
    /* #[wasm_bindgen(js_name = "glyph_hor_advance")]
    pub glyph_hor_advance: Option<u16>,

    #[wasm_bindgen(js_name = "glyph_ver_advance")]
    pub glyph_ver_advance: Option<u16>, */
    // / Checks that face has non-default variation coordinates.
    // #[cfg(feature = "variable-fonts")]
    // #[wasm_bindgen(js_name = "hasNonDefaultVariationCoordinates")]
    // pub has_non_default_variation_coordinates: bool,
}

#[wasm_bindgen]
impl TTFParser {
    /// Creates a new `TTFParser` from a raw data.
    ///
    /// `index` indicates the specific font face in a font collection.
    /// Use [`fonts_in_collection`] to get the total number of font faces.
    /// Defaults to 0 if not set.
    ///
    /// Required tables: `head`, `hhea` and `maxp`.
    ///
    /// If an optional table has invalid data it will be skipped.
    #[wasm_bindgen(constructor)]
    pub fn new(data: &[u8], index: Option<usize>) -> Result<TTFParser, JsError> {
        let face = match Face::parse(data, index.unwrap_or_default() as u32) {
            Ok(font) => font,
            Err(err) => return Err(JsError::new(&err.to_string())),
        };

        let tables = Tables::new(&face);
        // log(format!("{:?}", data).as_str());

        let permissions = tables.os2.map(|v| v.permissions);

        let parser = Self {
            tables,

            is_bold: face.is_bold(),
            is_italic: face.is_italic(),
            is_monospaced: face.is_monospaced(),
            is_oblique: face.is_oblique(),
            is_regular: face.is_regular(),
            is_subsetting_allowed: face.is_subsetting_allowed(),
            is_variable: face.is_variable(),
            // has_non_default_variation_coordinates: face.has_non_default_variation_coordinates(),
            units_per_em: face.units_per_em(),
            italic_angle: face.italic_angle(),

            ascender: face.ascender(),
            descender: face.descender(),
            height: face.height(),
            line_gap: face.line_gap(),
            capital_height: face.capital_height(),
            typographic_ascender: face.typographic_ascender(),
            typographic_descender: face.typographic_descender(),
            typographic_line_gap: face.typographic_line_gap(),
            vertical_ascender: face.vertical_ascender(),
            vertical_descender: face.vertical_descender(),
            vertical_height: face.vertical_height(),
            vertical_line_gap: face.vertical_line_gap(),
            x_height: face.x_height(),
            permissions,
            is_bitmap_embedding_allowed: face.is_bitmap_embedding_allowed(),
            number_of_glyphs: face.number_of_glyphs(),
            superscript_metrics: face.superscript_metrics().map(ScriptMetrics::from),
            // glyph_ver_advance: face.glyph_ver_advance(),
            // glyph_hor_advance: face.glyph_hor_advance(),
        };

        Ok(parser)
    }

    // #[wasm_bindgen(js_name = style)]
    // pub fn style(&self) -> tables::os2::Style {
    //     if let Some(os2_value) = self.tables.os2 {
    //         return os2_value.style;
    //     }
    //     tables::os2::Style::Normal
    // }

    /// Returns a bounding box that large enough to enclose any glyph from the face.
    #[wasm_bindgen(js_name = globalBoundingBox, method)]
    pub fn global_bounding_box(&self) -> tables::Rect {
        self.tables.head.global_bbox.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn tables(&mut self) -> Tables {
        self.tables.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn it_works() {
        // let bytes = fs::read("ABBvoice-Medium.ttf").unwrap();
        let _bytes = fs::read("NotoSansCJK.ttf").unwrap();
        /* let face = owned_ttf_parser::Face::parse(bytes.as_ref(), 0).unwrap();

        println!("units_per_em {:?}", face.units_per_em().to_string());

        let post_script_name = face
            .names()
            .into_iter()
            .find(|name| {
                name.name_id == owned_ttf_parser::name_id::POST_SCRIPT_NAME && name.is_unicode()
            })
            .and_then(|name| name.to_string());

        println!("postscript {:?}", post_script_name);

        let family_name = face
            .names()
            .into_iter()
            .find(|name| name.name_id == owned_ttf_parser::name_id::FAMILY && name.is_unicode())
            .and_then(|name| name.to_string());
        println!("family {:?}", family_name);

        let full_name = face
            .names()
            .into_iter()
            .find(|name| name.name_id == owned_ttf_parser::name_id::FULL_NAME && name.is_unicode())
            .and_then(|name| name.to_string());
        println!("full_name {:?}", full_name);

        let sub_family_name = face
            .names()
            .into_iter()
            .find(|name| name.name_id == owned_ttf_parser::name_id::SUBFAMILY && name.is_unicode())
            .and_then(|name| name.to_string());
        println!("sub_family_name {:?}", sub_family_name);

        println!(
            "typographic_line_gap {}",
            face.tables()
                .os2
                .unwrap()
                .typographic_line_gap()
                 .to_string()
        );

        println!("weight {}", face.tables().os2.unwrap().weight().to_number());

        println!(
            "glyph_hor_advance {}",
            face.glyph_hor_advance(face.glyph_index('N').unwrap())
                .unwrap()
        );

        println!(
            "glyph_ver_advance {}",
            face.glyph_ver_advance(face.glyph_index('N').unwrap())
                .unwrap()
        ); */

        // assert_eq!(face.height().to_le(), 4);
    }
}
