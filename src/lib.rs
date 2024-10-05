//! # Egui Colors
//!
//! Experimental toolkit to explore color styling in [`Egui`](https://github.com/emilk/egui)
//!
//! It is based on the [`Radix`](https://www.radix-ui.com/colors/docs/palette-composition/understanding-the-scale) system which maps a color scale to 12 functional
//! UI elements.
//! Scales (both light and dark mode) are computed and based on luminosity contrast algorithm defined by [`APCA`](https://github.com/Myndex).
//! Every scale uses one predefined `[u8; 3]` rgb color that is used as an accent color (if suitable).
//!
//!

pub(crate) mod apca;
pub(crate) mod scales;
pub mod tokens;
/// Some predefined themes
pub mod utils;

use scales::Scales;
use tokens::{ColorPreset, ColorTokens};
use utils::{LABELS, THEMES, THEME_NAMES};

/// The Colorix type is the main entry to this crate.
///
/// # Examples
///
/// ```
/// use egui::Context;
/// use egui_colors::{Colorix, tokens::ColorPreset};
/// //Define a colorix field in your egui App
/// #[derive(Default)]
/// struct App {
///     colorix: Colorix,
///     //..Default::default()
/// }
/// // initialize the Colorix with a theme
/// // a color theme is defined as [ColorPreset; 12]
/// // a ColorPreset is an enum with several preset colors and one Custom.
/// impl App {
///     fn new(ctx: &Context) -> Self {
///         let yellow_theme = [ColorPreset::Custom([232, 210, 7]); 12];
///         let colorix = Colorix::init(ctx, yellow_theme);
///         Self {
///             colorix,
///             ..Default::default()
///         }
///     }
/// }
/// ```
#[derive(Debug, Default, Clone)]
pub struct Colorix {
    pub tokens: ColorTokens,
    //pub tokens2: ColorTokens,
    pub(crate) theme: [ColorPreset; 12],
    theme_index: usize,
    pub(crate) scales: Scales,
}

impl Colorix {
    #[allow(clippy::must_use_candidate)]
    pub fn init(ctx: &egui::Context, theme: [ColorPreset; 12]) -> Self {
        let mut colorix = Self {
            theme,
            ..Default::default()
        };
        colorix.scales.dark_mode = ctx.style().visuals.dark_mode;
        colorix.get_theme_index();
        colorix.update_colors(ctx);
        colorix
    }

    fn get_theme_index(&mut self) {
        if let Some(i) = THEMES.iter().position(|t| t == &self.theme) {
            self.theme_index = i;
        };
    }
    /// WARNING: don't use the `light_dark` buttons that Egui provides.
    /// That will override the theme from this crate.
    pub fn light_dark_toggle_button(&mut self, ui: &mut egui::Ui) {
        #![allow(clippy::collapsible_else_if)]
        if ui.ctx().style().visuals.dark_mode {
            self.scales.dark_mode = true;
            if ui
                .add(
                    egui::Button::new(egui::RichText::new("☀").size(20.))
                        .min_size(egui::Vec2::new(30., 30.))
                        .frame(false),
                )
                .on_hover_text("Switch to light mode")
                .clicked()
            {
                self.scales.dark_mode = false;
                ui.ctx().set_visuals(egui::Visuals {
                    dark_mode: false,
                    ..Default::default()
                });
                self.update_colors(ui.ctx());
            }
        } else {
            if ui
                .add(
                    egui::Button::new(egui::RichText::new("🌙").size(20.))
                        .min_size(egui::Vec2::new(30., 30.))
                        .frame(false),
                )
                .on_hover_text("Switch to dark mode")
                .clicked()
            {
                self.scales.dark_mode = true;
                ui.ctx().set_visuals(egui::Visuals {
                    dark_mode: true,
                    ..Default::default()
                });
                self.update_colors(ui.ctx());
            }
        }
    }

    /// Choose from a list of preset themes. It is possible to add custom themes.
    /// NOTE: custom values chosen without the custom color picker are not recommended!
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use egui_colors::tokens::ColorPreset;
    /// let names = vec!["YellowGreen"];
    /// let themes = vec![[ColorPreset::Custom([178, 194, 31]); 12]];
    /// let custom = Some((names, themes));
    ///
    /// // if you want to display custom themes only, set `custom_only` to `true`
    /// app.colorix.themes_dropdown(ctx, ui, custom, false);
    /// ```
    pub fn themes_dropdown(
        &mut self,
        ui: &mut egui::Ui,
        custom_themes: Option<(Vec<&str>, Vec<[ColorPreset; 12]>)>,
        custom_only: bool,
    ) {
        let combi_themes: Vec<[ColorPreset; 12]>;
        let combi_names: Vec<&str>;

        if let Some(custom) = custom_themes {
            let (names, themes) = custom;
            if custom_only {
                combi_themes = themes;
                combi_names = names;
            } else {
                combi_themes = THEMES.iter().copied().chain(themes).collect();
                combi_names = THEME_NAMES.iter().copied().chain(names).collect();
            }
        } else {
            combi_names = THEME_NAMES.to_vec();
            combi_themes = THEMES.to_vec();
        }
        egui::ComboBox::from_id_salt("Select Theme")
            .selected_text(combi_names[self.theme_index])
            .show_ui(ui, |ui| {
                for i in 0..combi_themes.len() {
                    if ui
                        .selectable_value(&mut self.theme, combi_themes[i], combi_names[i])
                        .clicked()
                    {
                        self.theme_index = i;
                        self.update_colors(ui.ctx());
                    };
                }
            });
    }
    /// A widget with 12 dropdown menus of the UI elements (`ColorTokens`) that can be set.
    pub fn ui_combo_12(&mut self, ui: &mut egui::Ui) {
        let dropdown_colors: [ColorPreset; 23] = [
            ColorPreset::Gray,
            ColorPreset::EguiBlue,
            ColorPreset::Tomato,
            ColorPreset::Red,
            ColorPreset::Ruby,
            ColorPreset::Crimson,
            ColorPreset::Pink,
            ColorPreset::Plum,
            ColorPreset::Purple,
            ColorPreset::Violet,
            ColorPreset::Iris,
            ColorPreset::Indigo,
            ColorPreset::Blue,
            ColorPreset::Cyan,
            ColorPreset::Teal,
            ColorPreset::Jade,
            ColorPreset::Green,
            ColorPreset::Grass,
            ColorPreset::Brown,
            ColorPreset::Bronze,
            ColorPreset::Gold,
            ColorPreset::Orange,
            ColorPreset::Custom(self.scales.custom()),
        ];
        ui.vertical(|ui| {
            for (i, label) in LABELS.iter().enumerate() {
                ui.horizontal(|ui| {
                    let color_edit_size = egui::vec2(40.0, 18.0);
                    if let Some(ColorPreset::Custom(rgb)) = self.theme.get_mut(i) {
                        let re = ui.color_edit_button_srgb(rgb);
                        if re.changed() {
                            self.update_color(ui.ctx(), i);
                        }
                    } else {
                        // Allocate a color edit button's worth of space for non-custom presets,
                        // for alignment purposes.
                        ui.add_space(color_edit_size.x + ui.style().spacing.item_spacing.x);
                    }
                    egui::widgets::color_picker::show_color(
                        ui,
                        self.tokens.get_token(i),
                        color_edit_size,
                    );
                    egui::ComboBox::from_label(*label)
                        .selected_text(self.theme[i].label())
                        .show_ui(ui, |ui| {
                            for preset in dropdown_colors {
                                if ui
                                    .selectable_value(&mut self.theme[i], preset, preset.label())
                                    .clicked()
                                {
                                    self.update_color(ui.ctx(), i);
                                };
                            }
                        });
                });
            }
            ui.add_space(10.);
            if ui.button("Copy theme to clipboard").clicked() {
                ui.output_mut(|out| out.copied_text = format!("{:#?}", self.theme));
            }
        });
    }

    fn process_theme(&mut self) {
        let mut processed: Vec<usize> = vec![];
        for (i, v) in self.theme.iter().enumerate() {
            if !processed.contains(&i) {
                self.scales.process_color(*v);
                self.tokens.update_schema(i, self.scales.scale[i]);
                if i < self.theme.len() {
                    for (j, w) in self.theme[i + 1..].iter().enumerate() {
                        if w == v {
                            self.tokens
                                .update_schema(j + i + 1, self.scales.scale[j + i + 1]);
                            processed.push(j + i + 1);
                        }
                    }
                }
            }
        }
    }

    // pub fn process_2nd_theme(&mut self, theme: &[ColorPreset; 12]) {
    //     let mut processed: Vec<usize> = vec![];
    //     for (i, v) in theme.iter().enumerate() {
    //         if !processed.contains(&i) {
    //             self.scales.process_color(v);
    //             self.tokens2.update_scheme(i, self.scales.scale[i]);
    //             if i < self.theme.len() {
    //                 for (j, w) in self.theme[i + 1..].iter().enumerate() {
    //                     if w == v {
    //                         self.tokens2.update_scheme(j + i + 1, self.scales.scale[j + i + 1]);
    //                         processed.push(j + i + 1)
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     self.tokens.text_color();
    // }

    fn update_color(&mut self, ctx: &egui::Context, i: usize) {
        self.scales.process_color(self.theme[i]);
        self.tokens.update_schema(i, self.scales.scale[i]);
        self.tokens.set_text_color();
        self.tokens.set_egui_visuals(ctx);
    }

    fn update_colors(&mut self, ctx: &egui::Context) {
        self.process_theme();
        self.tokens.set_text_color();
        self.tokens.set_egui_visuals(ctx);
    }

    /// NOTE: values are clamped for useability.
    /// Creating custom themes outside these values is not recommended.
    pub fn custom_picker(&mut self, ui: &mut egui::Ui) {
        if egui::color_picker::color_edit_button_hsva(
            ui,
            &mut self.scales.custom,
            egui::color_picker::Alpha::Opaque,
        )
        .changed()
        {
            self.scales.clamp_custom();
        };
    }

    /// Set a background gradient. Choose 'true' for color from `solid_backgrounds`
    /// and 'false' for`ui_element_background`
    pub fn draw_background(&mut self, ctx: &egui::Context, accent: bool) {
        let bg = if accent {
            self.scales.process_color(self.theme[8]);
            self.scales.scale[2]
        } else {
            self.tokens.ui_element_background
        };
        let rect = egui::Context::available_rect(ctx);
        let layer_id = egui::LayerId::background();
        let painter = egui::Painter::new(ctx.clone(), layer_id, rect);
        let mut mesh = egui::Mesh::default();
        mesh.colored_vertex(rect.left_top(), self.tokens.app_background);
        mesh.colored_vertex(rect.right_top(), self.tokens.app_background);
        mesh.colored_vertex(rect.left_bottom(), bg);
        mesh.colored_vertex(rect.right_bottom(), bg);
        mesh.add_triangle(0, 1, 2);
        mesh.add_triangle(1, 2, 3);
        painter.add(egui::Shape::Mesh(mesh));
    }
    /// Returns the currently set theme
    #[must_use]
    pub const fn theme(&self) -> &[ColorPreset; 12] {
        &self.theme
    }
}
