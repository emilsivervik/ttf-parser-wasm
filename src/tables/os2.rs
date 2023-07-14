use super::{LineMetrics, Weight};
use ttf_parser::os2;
use wasm_bindgen::prelude::wasm_bindgen;

/// A face style.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum Style {
    /// A face that is neither italic not obliqued.
    Normal = "Normal",
    /// A form that is generally cursive in nature.
    Italic = "Italic",
    /// A typically-sloped version of the regular face.
    Oblique = "Oblique",
}

impl From<os2::Style> for Style {
    fn from(style: os2::Style) -> Style {
        match style {
            os2::Style::Italic => Style::Italic,
            os2::Style::Normal => Style::Normal,
            os2::Style::Oblique => Style::Oblique,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum Permissions {
    Installable = "Installable",
    Restricted = "Restricted",
    PreviewAndPrint = "PreviewAndPrint",
    Editable = "Editable",
    Malformed = "Malformed",
}

impl From<Option<os2::Permissions>> for Permissions {
    fn from(permission: Option<os2::Permissions>) -> Permissions {
        if let Some(v) = permission {
            match v {
                os2::Permissions::Editable => Permissions::Editable,
                os2::Permissions::Installable => Permissions::Installable,
                os2::Permissions::PreviewAndPrint => Permissions::PreviewAndPrint,
                os2::Permissions::Restricted => Permissions::Restricted,
            }
        } else {
            Permissions::Malformed
        }
    }
}

/// A face [width](https://docs.microsoft.com/en-us/typography/opentype/spec/os2#uswidthclass).
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum Width {
    UltraCondensed = "UltraCondensed",
    ExtraCondensed = "ExtraCondensed",
    Condensed = "Condensed",
    SemiCondensed = "SemiCondensed",
    Normal = "Normal",
    SemiExpanded = "SemiExpanded",
    Expanded = "Expanded",
    ExtraExpanded = "ExtraExpanded",
    UltraExpanded = "UltraExpanded",
}

impl From<os2::Width> for Width {
    fn from(width: os2::Width) -> Width {
        match width {
            os2::Width::Condensed => Width::Condensed,
            os2::Width::Expanded => Width::Expanded,
            os2::Width::ExtraCondensed => Width::ExtraCondensed,
            os2::Width::ExtraExpanded => Width::ExtraExpanded,
            os2::Width::Normal => Width::Normal,
            os2::Width::SemiCondensed => Width::SemiCondensed,
            os2::Width::SemiExpanded => Width::SemiExpanded,
            os2::Width::UltraCondensed => Width::UltraCondensed,
            os2::Width::UltraExpanded => Width::UltraExpanded,
        }
    }
}

#[wasm_bindgen(inspectable)]
#[derive(Clone, Copy)]
pub struct ScriptMetrics {
    /// Horizontal face size.
    pub x_size: i16,

    /// Vertical face size.
    pub y_size: i16,

    /// X offset.
    pub x_offset: i16,

    /// Y offset.
    pub y_offset: i16,
}

impl From<os2::ScriptMetrics> for ScriptMetrics {
    fn from(subscript_metrics: os2::ScriptMetrics) -> ScriptMetrics {
        ScriptMetrics {
            x_offset: subscript_metrics.x_offset,
            x_size: subscript_metrics.x_size,
            y_offset: subscript_metrics.y_offset,
            y_size: subscript_metrics.y_size,
        }
    }
}

/// A [OS/2 and Windows Metrics Table](https://docs.microsoft.com/en-us/typography/opentype/spec/os2).
#[wasm_bindgen(js_name = "OS2Table", getter_with_clone, inspectable)]
#[derive(Clone, Copy)]
pub struct Table {
    /// Returns weight class.
    pub weight: Weight,

    /// Returns face width.
    pub width: Width,

    /// Returns face permissions.
    pub permissions: Permissions,

    /// Checks if the face subsetting is allowed.
    #[wasm_bindgen(js_name = "isSubsettingAllowed")]
    pub is_subsetting_allowed: bool,

    /// Checks if the face bitmaps embedding is allowed.
    #[wasm_bindgen(js_name = "isBitmapEmbeddingAllowed")]
    pub is_bitmap_embedding_allowed: bool,

    /// Returns subscript metrics.
    #[wasm_bindgen(js_name = "subscriptMetrics")]
    pub subscript_metrics: ScriptMetrics,

    /// Returns superscript metrics.
    #[wasm_bindgen(js_name = "superscriptMetrics")]
    pub superscript_metrics: ScriptMetrics,

    /// Returns strikeout metrics.
    #[wasm_bindgen(js_name = "strikeoutMetrics")]
    pub strikeout_metrics: LineMetrics,

    // pub unicode_ranges: u128,
    /// Returns style.
    pub style: Style,

    /// Checks if face is bold.
    #[wasm_bindgen(js_name = "isBold")]
    pub is_bold: bool,

    /// Checks if typographic metrics should be used.
    #[wasm_bindgen(js_name = "useTypographicMetrics")]
    pub use_typographic_metrics: bool,

    /// Returns typographic ascender.
    #[wasm_bindgen(js_name = "typographicAscender")]
    pub typographic_ascender: i16,

    /// Returns typographic descender.
    #[wasm_bindgen(js_name = "typographicDescender")]
    pub typographic_descender: i16,

    /// Returns typographic line gap.
    #[wasm_bindgen(js_name = "typographicLineGap")]
    pub typographic_line_gap: i16,

    /// Returns Windows ascender.
    #[wasm_bindgen(js_name = "windowsAscender")]
    pub windows_ascender: i16,

    /// Returns Windows descender.
    #[wasm_bindgen(js_name = "windowsDescender")]
    pub windows_descender: i16,

    /// Returns x height.
    #[wasm_bindgen(js_name = "xHeight")]
    pub x_height: Option<i16>,

    /// Returns capital height.
    #[wasm_bindgen(js_name = "capitalHeight")]
    pub capital_height: Option<i16>,
}

impl Table {
    pub fn new(table: Option<os2::Table>) -> Option<Self> {
        if let Some(table) = table {
            let weight = table.weight();
            let width = table.width();
            let permissions: Permissions = table.permissions().into();
            let is_subsetting_allowed = table.is_subsetting_allowed();
            let is_bitmap_embedding_allowed = table.is_bitmap_embedding_allowed();
            let subscript_metrics = table.subscript_metrics();
            let superscript_metrics = table.superscript_metrics();
            let strikeout_metrics = table.strikeout_metrics();
            // let unicode_ranges = table.unicode_ranges(); TODO
            let style = table.style();
            let is_bold = table.is_bold();
            let use_typographic_metrics = table.use_typographic_metrics();
            let typographic_ascender = table.typographic_ascender();
            let typographic_descender = table.typographic_descender();
            let typographic_line_gap = table.typographic_line_gap();
            let windows_ascender = table.windows_ascender();
            let windows_descender = table.windows_descender();
            let x_height = table.x_height();
            let capital_height = table.capital_height();
            Some(Self {
                weight: weight.into(),
                width: width.into(),
                permissions,
                is_subsetting_allowed,
                is_bitmap_embedding_allowed,
                subscript_metrics: subscript_metrics.into(),
                superscript_metrics: superscript_metrics.into(),
                strikeout_metrics: strikeout_metrics.into(),
                style: style.into(),
                is_bold,
                use_typographic_metrics,
                typographic_ascender,
                typographic_descender,
                typographic_line_gap,
                windows_ascender,
                windows_descender,
                x_height,
                capital_height,
            })
        } else {
            None
        }
    }
}
