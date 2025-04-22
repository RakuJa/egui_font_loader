use egui::epaint::text::FontInsert;
use egui::epaint::text::FontPriority;
use egui::epaint::text::InsertFontFamily;
use egui::FontData;
use egui::FontFamily;
use std::fs;
use std::path::Path;
use thiserror::Error;

/// Struct used for font loading operation using the `load_fonts` function.
///
/// # Examples
/// ```rust
/// use egui_font_loader::LoaderFontData;
///
/// let roboto = LoaderFontData {
///     name: "Roboto".into(),
///     path: "assets/fonts/Roboto.ttf".to_string(),
/// };
/// ```
pub struct LoaderFontData {
    pub name: String,
    pub path: String,
}

#[derive(Error, Debug)]
pub enum LoadFontError {
    #[error("Font file not found at path: {0}")]
    FileNotFound(String),

    #[error("Failed to read font file at {path}: {source}")]
    FileReadError {
        path: String,
        #[source]
        source: std::io::Error,
    },
}

/// # Examples
/// ``` rust
/// load_font!(ctx, "DesiredFontName", "path/to/font.ttf");
/// ```
/// # Panics
/// The function may panic if the file cannot be found/read or egui refuses it (bad format usually)
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

/// # Errors
///
/// Will return `Err` if any of the elements of `fonts` either does not exists or cannot be read.
///
/// # Examples
/// ``` rust
/// use egui_font_loader::LoaderFontData;
/// use egui_font_loader::load_fonts;
/// let fonts = vec![
///     LoaderFontData {
///         name: "DesiredFontName".into(),
///         path: "path/to/custom/font/first.ttf".into(),
///     },
///     LoaderFontData {
///         name: "SecondCustomFont".into(),
///         path: "path/to/custom/font/second.ttf".into(),
///     },
/// ];
/// load_fonts(&egui_ctx, fonts).unwrap();
/// ```
///
/// # Panics
/// The function may panic if, after having correctly loaded a font, egui refuses it (bad format usually)
pub fn load_fonts(
    ctx: &egui::Context,
    fonts: impl IntoIterator<Item = LoaderFontData>,
) -> Result<(), LoadFontError> {
    for font in fonts {
        let path = Path::new(&font.path);
        if !path.exists() {
            return Err(LoadFontError::FileNotFound(font.path));
        }
        let file_data = fs::read(path).map_err(|e| LoadFontError::FileReadError {
            path: font.path,
            source: e,
        })?;
        ctx.add_font(FontInsert {
            families: vec![InsertFontFamily {
                family: FontFamily::Name(font.name.as_str().into()),
                priority: FontPriority::Lowest,
            }],
            name: font.name,
            data: FontData::from_owned(file_data),
        });
    }
    Ok(())
}
