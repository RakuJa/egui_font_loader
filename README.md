# egui_font_loader
A simple library to simplify egui font loading


## How to load fonts

Important! It's advised to load the font only once and avoid doing it in the update function.

Fonts need to be found locally.

### Single font

To load a single font known at compile time, it's suggested to use the load_font macro
```rust
load_font!(ctx, "DesiredFontName", "path/to/font.ttf");
```
The DesiredFontName can be anything, just keep it in mind to load it later on.

### Multiple fonts at once
```rust
use egui_font_loader::LoaderFontData;
use egui_font_loader::load_fonts;
let fonts = vec![
    LoaderFontData {
        name: "DesiredFontName".into(),
        path: "path/to/custom/font/first.ttf".into(),
    },
    LoaderFontData {
        name: "SecondCustomFont".into(),
        path: "path/to/custom/font/second.ttf".into(),
    },
];
load_fonts(&egui_ctx, fonts).unwrap();
```
## How to use the loaded font
```rust
// Some code
ui.heading(
    RichText::new("Code By RakuJa")
        .color(Color32::from_rgb(102, 0, 51))
        .font(FontId {
            size: 15.0,
            family: FontFamily::Name("DesiredFontName".into()),
        }),
);
// Some other code
```

## How to load only once the desired fonts

In the main.rs
```rust
eframe::run_native(
    "My window title",
    options,
    Box::new(|cc| {
        Ok(Box::new(MyApp::new(cc)))
    }),
)

```
In myapp.rs
```rust
impl MyApp {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
    ) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        load_fonts(&cc.egui_ctx, vec![/*add LoaderFontData here*/]).unwrap();
        Self {//initialize it}
    }
}
```
