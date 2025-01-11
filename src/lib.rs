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
pub mod apply_to;
pub(crate) mod scales;
pub mod tokens;
/// Some predefined themes
pub mod utils;

use scales::Scales;
use tokens::{ColorTokens, ThemeColor};
use utils::{LABELS, THEMES, THEME_NAMES};

pub use apply_to::ApplyTo;

/// A set of colors that are used together to set a visual feel for the ui
pub type Theme = [ThemeColor; 12];

/// The Colorix type is the main entry to this crate.
///
/// # Examples
///
/// ```
/// use egui::Context;
/// use egui_colors::{Colorix, tokens::ThemeColor};
/// //Define a colorix field in your egui App
/// #[derive(Default)]
/// struct App {
///     colorix: Colorix,
///     //..Default::default()
/// }
/// // initialize the Colorix with a theme
/// // a color theme is defined as [ThemeColor; 12]
/// // a ThemeColor is an enum with several preset colors and one Custom.
/// impl App {
///     fn new(ctx: &Context) -> Self {
///         let yellow_theme = [ThemeColor::Custom([232, 210, 7]); 12];
///         let colorix = Colorix::init(ctx, yellow_theme);
///         colorix.apply_global(ctx);
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
    pub(crate) theme: Theme,
    theme_index: usize,
    pub(crate) scales: Scales,
}

impl Colorix {
    /// Initialize the Colorix with the dark mode setting pulled from the Egui
    /// context
    #[allow(clippy::must_use_candidate)]
    pub fn init(ctx: &egui::Context, theme: Theme) -> Self {
        Self::init_with_dark_mode(theme, ctx.style().visuals.dark_mode)
    }

    /// Initialize the Colorix with a dark mode setting
    #[allow(clippy::must_use_candidate)]
    pub fn init_with_dark_mode(theme: Theme, is_dark_mode: bool) -> Self {
        let mut colorix = Self {
            theme,
            ..Default::default()
        };
        colorix.scales.dark_mode = is_dark_mode;
        colorix.get_theme_index();
        colorix.update_colors();
        colorix
    }

    /// Applies the theme to the global Egui context
    pub fn apply_global(&self, ctx: &egui::Context) {
        self.tokens.set_global_egui_visuals(ctx);
    }

    /// Applies the theme to the local widget tree only
    pub fn apply_local(&self, ui: &mut egui::Ui) {
        self.tokens.set_local_egui_visuals(ui, self.scales.dark_mode);
    }

    fn get_theme_index(&mut self) {
        if let Some(i) = THEMES.iter().position(|t| t == &self.theme) {
            self.theme_index = i;
        };
    }

    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn is_dark(&self) -> bool {
        self.scales.dark_mode
    }

    pub fn set_dark(&mut self, ui: &mut egui::Ui, apply_to: ApplyTo) {
        self.scales.dark_mode = true;
        apply_to.set_visuals(ui, egui::Visuals {
            dark_mode: true,
            ..Default::default()
        });
        self.update_colors();
        apply_to.apply(ui, self);
    }

    pub fn set_light(&mut self, ui: &mut egui::Ui, apply_to: ApplyTo) {
        self.scales.dark_mode = false;
        apply_to.set_visuals(ui, egui::Visuals {
            dark_mode: false,
            ..Default::default()
        });
        self.update_colors();
        apply_to.apply(ui, self);
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
                ApplyTo::Global.set_visuals(ui, egui::Visuals {
                    dark_mode: false,
                    ..Default::default()
                });
                self.update_colors();
                ApplyTo::Global.apply(ui, self);
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
                ApplyTo::Global.set_visuals(ui, egui::Visuals {
                    dark_mode: true,
                    ..Default::default()
                });
                self.update_colors();
                ApplyTo::Global.apply(ui, self);
            }
        }
    }

    /// Choose from a list of preset themes. It is possible to add custom themes.
    /// NOTE: custom values chosen without the custom color picker are not recommended!
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use egui_colors::tokens::ThemeColor;
    /// let names = vec!["YellowGreen"];
    /// let themes = vec![[ThemeColor::Custom([178, 194, 31]); 12]];
    /// let custom = Some((names, themes));
    ///
    /// // if you want to display custom themes only, set `custom_only` to `true`
    /// app.colorix.themes_dropdown(ui, custom, false);
    /// ```
    pub fn themes_dropdown(
        &mut self,
        ui: &mut egui::Ui,
        custom_themes: Option<(Vec<&str>, Vec<Theme>)>,
        custom_only: bool,
        apply_to: ApplyTo,
    ) {
        let combi_themes: Vec<Theme>;
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
                        self.update_colors();
                        apply_to.apply(ui, self);
                    };
                }
            });
    }
    /// A widget with 12 dropdown menus of the UI elements (`ColorTokens`) that can be set.
    /// Add copy: true to display a button to copy the theme in debug format
    pub fn ui_combo_12(&mut self, ui: &mut egui::Ui, copy: bool, apply_to: ApplyTo) {
        let dropdown_colors: [ThemeColor; 23] = [
            ThemeColor::Gray,
            ThemeColor::EguiBlue,
            ThemeColor::Tomato,
            ThemeColor::Red,
            ThemeColor::Ruby,
            ThemeColor::Crimson,
            ThemeColor::Pink,
            ThemeColor::Plum,
            ThemeColor::Purple,
            ThemeColor::Violet,
            ThemeColor::Iris,
            ThemeColor::Indigo,
            ThemeColor::Blue,
            ThemeColor::Cyan,
            ThemeColor::Teal,
            ThemeColor::Jade,
            ThemeColor::Green,
            ThemeColor::Grass,
            ThemeColor::Brown,
            ThemeColor::Bronze,
            ThemeColor::Gold,
            ThemeColor::Orange,
            ThemeColor::Custom(self.scales.custom()),
        ];
        ui.vertical(|ui| {
            for (i, label) in LABELS.iter().enumerate() {
                ui.horizontal(|ui| {
                    let color_edit_size = egui::vec2(40.0, 18.0);
                    if let Some(ThemeColor::Custom(rgb)) = self.theme.get_mut(i) {
                        let re = ui.color_edit_button_srgb(rgb);
                        if re.changed() {
                            self.update_color(i);
                            apply_to.apply(ui, self);
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
                                    self.update_color(i);
                                    apply_to.apply(ui, self);
                                };
                            }
                        });
                });
            }
            if copy {
                ui.add_space(10.);
                if ui.button("Copy theme to clipboard").clicked() {
                    ui.output_mut(|out| out.copied_text = format!("{:#?}", self.theme));
                }
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

    fn update_color(&mut self, i: usize) {
        self.scales.process_color(self.theme[i]);
        self.tokens.update_schema(i, self.scales.scale[i]);
        self.tokens.color_on_accent();
    }

    fn update_colors(&mut self) {
        self.process_theme();
        self.tokens.color_on_accent();
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
    pub const fn theme(&self) -> &Theme {
        &self.theme
    }
}
