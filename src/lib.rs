use anyhow::Result;
use egui::epaint::text::FontInsert;
use egui::epaint::text::FontPriority;
use egui::epaint::text::InsertFontFamily;
use egui::FontData;
use egui::FontFamily;
use std::fs;

pub struct LoaderFontData {
    pub name: String,
    pub path: String,
}

#[macro_export]
macro_rules! load_font {
    ($ctx: expr, $($name:literal, $path:literal),*) => {
        $(
        $ctx.add_font(
            FontInsert {
                name: $name.into(),
                data: FontData::from_static(include_bytes!($path)).into(),
                families: vec![InsertFontFamily {
                    family: FontFamily::Name($name.into()),
                    priority: FontPriority::Lowest
                }]
            }
        );
        )*
    };
}

pub fn load_fonts(
    ctx: &egui::Context,
    fonts: impl IntoIterator<Item = LoaderFontData>,
) -> Result<()> {
    for font in fonts {
        ctx.add_font(FontInsert {
            families: vec![InsertFontFamily {
                family: FontFamily::Name(font.name.as_str().into()),
                priority: FontPriority::Lowest,
            }],
            name: font.name,
            data: FontData::from_owned(fs::read(font.path)?),
        });
    }
    Ok(())
}
