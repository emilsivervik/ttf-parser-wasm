use tsify::Tsify;
use ttf_parser::{name, LazyArray16};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

/// A [Name ID](https://docs.microsoft.com/en-us/typography/opentype/spec/name#name-ids).
#[wasm_bindgen(skip_typescript)]
#[derive(Clone, Tsify)]
pub enum NameId {
    CopyrightNotice = "CopyrightNotice",
    Family = "Family",
    Subfamily = "Subfamily",
    UniqueId = "UniqueId",
    FullName = "FullName",
    Version = "Version",
    PostScriptName = "PostScriptName",
    Trademark = "Trademark",
    Manufacturer = "Manufacturer",
    Designer = "Designer",
    Description = "Description",
    VendorUrl = "VendorUrl",
    DesignerUrl = "DesignerUrl",
    License = "License",
    LicenseUrl = "LicenseUrl",
    // Reserved = "Reserved",
    TypographicFamily = "TypographicFamily",
    TypographicSubFamily = "TypographicSubFamily",
    CompatibleFull = "CompatibleFull",
    SampleText = "SampleText",
    PostScriptCID = "PostScriptCID",
    WWSFamily = "WWSFamily",
    WWSSubFamily = "WWSSubFamily",
    LightBackgroundPalette = "LightBackgroundPalette",
    DarkBackgroundPalette = "DarkBackgroundPalette",
    VariationsPostScriptNamePrefix = "VariationsPostScriptNamePrefix",
    Unknown = "Unknown",
}

impl From<u16> for NameId {
    fn from(name_id: u16) -> NameId {
        match name_id {
            name::name_id::COPYRIGHT_NOTICE => NameId::CopyrightNotice,
            name::name_id::FAMILY => NameId::Family,
            name::name_id::SUBFAMILY => NameId::Subfamily,
            name::name_id::UNIQUE_ID => NameId::UniqueId,
            name::name_id::FULL_NAME => NameId::FullName,
            name::name_id::VERSION => NameId::Version,
            name::name_id::POST_SCRIPT_NAME => NameId::PostScriptName,
            name::name_id::TRADEMARK => NameId::Trademark,
            name::name_id::MANUFACTURER => NameId::Manufacturer,
            name::name_id::DESIGNER => NameId::Designer,
            name::name_id::DESCRIPTION => NameId::Description,
            name::name_id::VENDOR_URL => NameId::VendorUrl,
            name::name_id::DESIGNER_URL => NameId::DesignerUrl,
            name::name_id::LICENSE => NameId::License,
            name::name_id::LICENSE_URL => NameId::LicenseUrl,
            name::name_id::TYPOGRAPHIC_FAMILY => NameId::TypographicFamily,
            name::name_id::TYPOGRAPHIC_SUBFAMILY => NameId::TypographicSubFamily,
            name::name_id::COMPATIBLE_FULL => NameId::CompatibleFull,
            name::name_id::SAMPLE_TEXT => NameId::SampleText,
            name::name_id::POST_SCRIPT_CID => NameId::PostScriptCID,
            name::name_id::WWS_FAMILY => NameId::WWSFamily,
            name::name_id::WWS_SUBFAMILY => NameId::WWSSubFamily,
            name::name_id::LIGHT_BACKGROUND_PALETTE => NameId::LightBackgroundPalette,
            name::name_id::DARK_BACKGROUND_PALETTE => NameId::DarkBackgroundPalette,
            name::name_id::VARIATIONS_POST_SCRIPT_NAME_PREFIX => {
                NameId::VariationsPostScriptNamePrefix
            }
            _ => NameId::Unknown,
        }
    }
}

/// A [platform ID](https://docs.microsoft.com/en-us/typography/opentype/spec/name#platform-ids).
#[wasm_bindgen(skip_typescript)]
// #[derive(Clone, Copy)]
#[derive(Clone, Tsify)]
pub enum PlatformId {
    Unicode = "Unicode",
    Macintosh = "Macintosh",
    Iso = "Iso",
    Windows = "Windows",
    Custom = "Custome",
}

impl From<name::PlatformId> for PlatformId {
    fn from(platform_id: name::PlatformId) -> PlatformId {
        match platform_id {
            name::PlatformId::Custom => PlatformId::Custom,
            name::PlatformId::Iso => PlatformId::Iso,
            name::PlatformId::Macintosh => PlatformId::Macintosh,
            name::PlatformId::Unicode => PlatformId::Unicode,
            name::PlatformId::Windows => PlatformId::Windows,
        }
    }
}

/// A [Name Record](https://docs.microsoft.com/en-us/typography/opentype/spec/name#name-records).
#[wasm_bindgen(inspectable, skip_typescript, js_name = "INameRecord")]
#[derive(Clone, Tsify)]
pub struct NameRecord {
    /// A platform ID.
    #[wasm_bindgen(js_name = "platformId")]
    #[wasm_bindgen(getter_with_clone)]
    pub platform_id: PlatformId,

    /// A language ID.
    #[wasm_bindgen(js_name = "languageId")]
    #[wasm_bindgen(getter_with_clone)]
    pub language_id: String,

    /// A [Name ID](https://docs.microsoft.com/en-us/typography/opentype/spec/name#name-ids).
    #[wasm_bindgen(js_name = "nameId", getter_with_clone)]
    pub name_id: NameId,

    #[wasm_bindgen(getter_with_clone)]
    pub name: String,
}

#[wasm_bindgen(typescript_custom_section)]
const ITEXT_STYLE: &'static str = r#"
type StringList = NameRecord[]
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "StringList")]
    #[derive(Clone)]
    pub type StringList;
}

/// A [Naming Table](https://docs.microsoft.com/en-us/typography/opentype/spec/name).
#[wasm_bindgen(js_name = "NAMETable", inspectable)]
#[derive(Clone)]
pub struct Table {
    #[wasm_bindgen(getter_with_clone, typescript_type = "StringList")]
    pub names: StringList,
}

impl Table {
    pub fn new(table: Option<name::Table>) -> Option<Self> {
        let Some(table_names) = table else {
            return None;
        };
        let names = table_names
            .names
            .into_iter()
            .map(|v| {
                let name: String = {
                    if v.is_unicode() {
                        let mut name: Vec<u16> = Vec::new();
                        for c in LazyArray16::<u16>::new(v.name) {
                            name.push(c);
                        }

                        String::from_utf16(&name).unwrap()
                    } else {
                        std::str::from_utf8(v.name).unwrap_or_default().to_string()
                    }
                };

                NameRecord {
                    platform_id: v.platform_id.into(),
                    language_id: v.language().to_string(),
                    name_id: v.name_id.into(),
                    name,
                }
            })
            .map(JsValue::from)
            .collect::<Vec<JsValue>>();

        let test = StringList::from(serde_wasm_bindgen::to_value(&names).unwrap());

        Some(Self {
            names: StringList::from(JsValue::from(&names).unwrap()),
        })
    }
}
