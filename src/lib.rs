//! # egui Colors
//!
//! Experimental toolkit to explore color styling in [`egui`](https://github.com/emilk/egui)
//!
//! Based on the [`Radix`](https://www.radix-ui.com/colors/docs/palette-composition/understanding-the-scale) system which maps a color scale to 12 functional
//! UI elements.
//! Scales (both light and dark mode) are computed with luminosity contrast algorithm defined by [`APCA`](https://github.com/Myndex).
//! Every scale uses one predefined `[u8; 3]` rgb color that is used as an accent color (if suitable).
//!
//!

pub(crate) mod animator;
pub(crate) mod apca;
pub(crate) mod color_space;
pub(crate) mod scales;
pub mod tokens;
/// Some predefined themes
pub mod utils;

use animator::ColorAnimator;
use egui::{Context, Ui};
use scales::Scales;
use std::sync::Arc;
use tokens::{ColorTokens, ThemeColor};
use utils::{LABELS, THEMES, THEME_NAMES};

/// A set of colors that are used together to set a visual feel for the ui
pub type Theme = [ThemeColor; 12];

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ApplyTo {
    Global,
    Local,
    #[default]
    ExtraScale,
}

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
///  // You can choose between different scopes: global, local or extra_set.
/// impl App {
///     fn new(ctx: &Context) -> Self {
///         let yellow_theme = [ThemeColor::Custom([232, 210, 7]); 12];
///         let colorix = Colorix::global(ctx, yellow_theme);
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
    animated: bool,
    pub animator: ColorAnimator,
    pub(crate) apply_to: ApplyTo,
}

impl Colorix {
    #[must_use]
    pub fn global(ctx: &Context, theme: Theme) -> Self {
        let mut colorix = Self {
            theme,
            ..Default::default()
        };
        let mode = ctx.style().visuals.dark_mode;
        colorix.apply_to = ApplyTo::Global;
        colorix.tokens.apply_to = ApplyTo::Global;
        colorix.set_colorix_mode(mode);
        colorix.get_theme_index();
        colorix.update_colors(Some(ctx), None);
        colorix
    }
    /// Initialize a Colorix instance that applies to local ui.
    /// It needs an `update_locally(ui)` in the egui `update` function to work.
    pub fn local(ui: &mut Ui, theme: Theme) -> Self {
        let mut colorix = Self {
            theme,
            ..Default::default()
        };
        let mode = ui.ctx().style().visuals.dark_mode;
        colorix.set_colorix_mode(mode);
        colorix.get_theme_index();
        colorix.apply_to = ApplyTo::Local;
        colorix.tokens.apply_to = ApplyTo::Local;
        colorix.update_colors(None, Some(ui));
        colorix
    }
    /// Initialize a colorix to provide extra scale. It doesn't apply to any ui.
    #[must_use]
    pub fn extra_scale(ctx: &Context, theme: Theme) -> Self {
        let mut colorix = Self {
            theme,
            ..Default::default()
        };
        let mode = ctx.style().visuals.dark_mode;
        colorix.set_colorix_mode(mode);
        colorix.get_theme_index();
        colorix.apply_to = ApplyTo::ExtraScale;
        colorix.tokens.apply_to = ApplyTo::ExtraScale;
        colorix.update_colors(Some(ctx), None);
        colorix
    }
    #[must_use]
    pub fn local_from_style(theme: Theme, dark_mode: bool) -> Self {
        let mut colorix = Self::default();
        colorix.set_colorix_mode(dark_mode);
        colorix.theme = theme;
        colorix.apply_to = ApplyTo::Local;
        colorix.tokens.apply_to = ApplyTo::Local;
        colorix.update_colors(None, None);
        colorix
    }
    /// Set animator
    /// # Example
    ///
    /// ```ignore
    /// impl App {
    ///     fn new(ctx: &Context) -> Self {
    ///         let yellow_theme = [ThemeColor::Custom([232, 210, 7]); 12];
    ///         let colorix = Colorix::global(ctx, yellow_theme).animated().set_time(2.0);
    ///         Self {
    ///             colorix,
    ///             ..Default::default()
    ///         }
    ///     }
    /// }
    /// ```
    #[must_use]
    pub const fn animated(mut self) -> Self {
        self.animated = true;
        self.init_animator();
        self
    }
    /// Change the default time (1.0) of the animation.
    #[must_use]
    pub const fn set_time(mut self, new_time: f32) -> Self {
        if self.animated {
            self.animator.set_time(new_time);
        }
        self
    }

    /// sets new theme and animates towards it.
    pub fn update_theme(&mut self, ctx: &egui::Context, theme: Theme) {
        self.theme = theme;
        self.get_theme_index();
        self.update_colors(Some(ctx), None);
    }
    // starts animation; has only visible effect on the tokenshifts.
    pub fn shift_tokens(&mut self, ctx: &egui::Context) {
        self.animator.start(ctx);
    }
    #[must_use]
    pub const fn dark_mode(&self) -> bool {
        self.scales.dark_mode
    }

    const fn init_animator(&mut self) {
        self.animator = ColorAnimator::new(self.tokens);
        self.animator.apply_to = self.apply_to;
    }

    /// Necessary to engage the color animation
    /// works only for global ui and `extra_scale`
    pub fn set_animator(&mut self, ctx: &Context) {
        match self.apply_to {
            ApplyTo::Global | ApplyTo::ExtraScale => {
                if self.animated {
                    self.animator.set_animate(Some(ctx), None, self.tokens);
                }
            }
            ApplyTo::Local => {}
        }
    }

    fn get_theme_index(&mut self) {
        if let Some(i) = THEMES.iter().position(|t| t == &self.theme) {
            self.theme_index = i;
        }
    }
    /// create theme based on 1 custom color from color picker
    pub fn twelve_from_custom(&mut self, ui: &mut Ui) {
        self.theme = [ThemeColor::Custom(self.scales.custom()); 12];
        self.match_and_update_colors(ui);
    }

    fn match_and_update_colors(&mut self, ui: &mut Ui) {
        match self.apply_to {
            ApplyTo::Global | ApplyTo::ExtraScale => {
                self.update_colors(Some(ui.ctx()), None);
            }
            ApplyTo::Local => {
                self.update_colors(None, Some(ui));
            }
        }
    }
    const fn set_colorix_mode(&mut self, mode: bool) {
        self.scales.dark_mode = mode;
        self.tokens.dark_mode = mode;
    }
    /// If you initialize a Colorix instance with a `local` scope, this needs to be placed
    /// inside the egui `update` fn.
    pub fn update_locally(&mut self, ui: &mut Ui) {
        if self.apply_to == ApplyTo::Local {
            if self.animated {
                self.animator.set_animate(None, Some(ui), self.tokens);
            } else {
                self.update_colors(None, Some(ui));
            }
        }
    }

    fn set_ui_mode(&self, ui: &mut Ui, mode: bool) {
        match self.apply_to {
            ApplyTo::Global => ui.ctx().style_mut(|style| style.visuals.dark_mode = mode),
            ApplyTo::Local => ui.style_mut().visuals.dark_mode = mode,
            ApplyTo::ExtraScale => {}
        }
    }
    /// Change the color mode to dark.
    pub fn set_dark(&mut self, ui: &mut Ui) {
        self.set_colorix_mode(true);
        self.set_ui_mode(ui, true);
        self.match_and_update_colors(ui);
    }
    /// Change the color mode to light.
    pub fn set_light(&mut self, ui: &mut Ui) {
        self.set_colorix_mode(false);
        self.set_ui_mode(ui, false);
        self.match_and_update_colors(ui);
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

    fn match_egui_visuals(&self, ui: &mut Ui) {
        match self.apply_to {
            ApplyTo::Global => self.tokens.set_ctx_visuals(ui.ctx()),
            ApplyTo::Local => self.tokens.set_ui_visuals(ui),
            ApplyTo::ExtraScale => {}
        }
    }

    fn update_color(&mut self, ui: &mut Ui, i: usize) {
        self.scales.process_color(self.theme[i]);
        self.tokens.update_schema(i, self.scales.scale[i]);
        self.tokens.color_on_accent();
        if self.animated {
            self.animator.start(ui.ctx());
        } else {
            self.match_egui_visuals(ui);
        }
    }

    fn update_colors(&mut self, ctx: Option<&Context>, ui: Option<&mut Ui>) {
        if self.animated {
            self.process_theme();
            self.tokens.color_on_accent();
            if let Some(ctx) = ctx {
                self.animator.start(ctx);
            } else if let Some(ui) = ui {
                self.animator.start(ui.ctx());
            }
        } else {
            self.process_theme();
            self.tokens.color_on_accent();
            if let Some(ctx) = ctx {
                if self.apply_to != ApplyTo::ExtraScale {
                    self.tokens.set_ctx_visuals(ctx);
                }
            } else if let Some(ui) = ui {
                self.tokens.set_ui_visuals(ui);
            }
        }
    }

    /// WARNING: don't use the `light_dark` buttons that egui provides.
    /// That will override the themes from this crate. It needs the size for the button in f32
    pub fn light_dark_toggle_button(&mut self, ui: &mut Ui, size: f32) {
        #![allow(clippy::collapsible_else_if)]
        if self.dark_mode() {
            if ui
                .add(egui::Button::new(egui::RichText::new("☀").size(size)).frame(false))
                .on_hover_text("Switch to light mode")
                .clicked()
            {
                self.set_colorix_mode(false);
                self.set_ui_mode(ui, false);
                self.match_and_update_colors(ui);
            }
        } else {
            if ui
                .add(egui::Button::new(egui::RichText::new("🌙").size(size)).frame(false))
                .on_hover_text("Switch to dark mode")
                .clicked()
            {
                self.set_colorix_mode(true);
                self.set_ui_mode(ui, true);
                self.match_and_update_colors(ui);
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
        ui: &mut Ui,
        custom_themes: Option<(Vec<&str>, Vec<Theme>)>,
        custom_only: bool,
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
        egui::ComboBox::from_id_salt(ui.id())
            .selected_text(combi_names[self.theme_index])
            .show_ui(ui, |ui| {
                for i in 0..combi_themes.len() {
                    if ui
                        .selectable_value(&mut self.theme, combi_themes[i], combi_names[i])
                        .clicked()
                    {
                        self.theme_index = i;
                        self.match_and_update_colors(ui);
                    }
                }
            });
    }
    /// A widget with 12 dropdown menus of the UI elements (`ColorTokens`) that can be set.
    pub fn ui_combo_12(&mut self, ui: &mut Ui, copy: bool) {
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
                            self.update_color(ui, i);
                        }
                    } else {
                        // Allocate a color edit button's worth of space for non-custom presets,
                        // for alignment purposes.
                        ui.add_space(color_edit_size.x + ui.style().spacing.item_spacing.x);
                    }
                    let color = if self.animated {
                        self.animator.animated_tokens.get_token(i)
                    } else {
                        self.tokens.get_token(i)
                    };
                    egui::widgets::color_picker::show_color(ui, color, color_edit_size);
                    egui::ComboBox::from_label(*label)
                        .selected_text(self.theme[i].label())
                        .show_ui(ui, |ui| {
                            for preset in dropdown_colors {
                                if ui
                                    .selectable_value(&mut self.theme[i], preset, preset.label())
                                    .clicked()
                                {
                                    self.update_color(ui, i);
                                }
                            }
                        });
                });
            }
        });
        if copy {
            ui.add_space(10.);
            if ui.button("Copy theme to clipboard").clicked() {
                ui.ctx().copy_text(format!("{:#?}", self.theme));
            }
        }
    }

    /// NOTE: values are clamped for useability.
    /// Creating custom themes outside these values is not recommended.
    pub fn custom_picker(&mut self, ui: &mut Ui) {
        if egui::color_picker::color_edit_button_hsva(
            ui,
            &mut self.scales.custom,
            egui::color_picker::Alpha::Opaque,
        )
        .changed()
        {
            self.scales.clamp_custom();
        }
    }

    /// Set a background gradient. Choose 'true' for color from `solid_backgrounds` (if animated `active_ui_element_background`)
    /// and 'false' for `ui_element_background`
    pub fn draw_background(&mut self, ctx: &Context, accent: bool) {
        let (ui_element, background) = if self.animated {
            (
                self.animator.animated_tokens.ui_element_background,
                self.animator.animated_tokens.app_background,
            )
        } else {
            (
                self.tokens.ui_element_background,
                self.tokens.app_background,
            )
        };
        let bg = if accent {
            self.animator.animated_tokens.active_ui_element_background
        } else {
            ui_element
        };
        let rect = egui::Context::available_rect(ctx);
        let layer_id = egui::LayerId::background();
        let painter = egui::Painter::new(ctx.clone(), layer_id, rect);
        let mut mesh = egui::Mesh::default();
        mesh.colored_vertex(rect.left_top(), background);
        mesh.colored_vertex(rect.right_top(), background);
        mesh.colored_vertex(rect.left_bottom(), bg);
        mesh.colored_vertex(rect.right_bottom(), bg);
        mesh.add_triangle(0, 1, 2);
        mesh.add_triangle(1, 2, 3);
        painter.add(egui::Shape::Mesh(Arc::new(mesh)));
    }
    /// Returns the currently set theme
    #[must_use]
    pub const fn theme(&self) -> &Theme {
        &self.theme
    }
}
