use crate::apca::estimate_lc;
use egui::{
    self,
    style::{TextCursorStyle, WidgetVisuals},
    Color32, Rounding, Stroke,
};
use palette::{LinSrgb, Srgb};

/// The functional UI elements mapped to a scale
#[derive(Default, Debug, Clone, Copy)]
pub struct ColorTokens {
    pub app_background: Color32,
    pub subtle_background: Color32,
    pub ui_element_background: Color32,
    pub hovered_ui_element_background: Color32,
    pub active_ui_element_background: Color32,
    pub subtle_borders_and_separators: Color32,
    pub ui_element_border_and_focus_rings: Color32,
    pub hovered_ui_element_border: Color32,
    pub solid_backgrounds: Color32,
    pub hovered_solid_backgrounds: Color32,
    pub low_contrast_text: Color32,
    pub high_contrast_text: Color32,
    pub(crate) inverse_color: bool,
    pub(crate) text_color: Color32,
}

impl ColorTokens {
    pub(crate) fn set_text_color(&mut self) {
        let lc = estimate_lc(egui::Color32::WHITE, self.solid_backgrounds);
        if lc > -46. {
            self.inverse_color = true;
            let mut hsva: egui::ecolor::Hsva = self.solid_backgrounds.into();
            hsva.s = 0.7;
            hsva.v = 0.01;
            self.text_color = hsva.into();
        } else {
            self.text_color = egui::Color32::WHITE;
        }
    }

    /// the color for the text on accented color (solid backgrounds)
    #[must_use]
    pub const fn text_color(&self) -> Color32 {
        self.text_color
    }

    /// notifies when lc > -46.
    #[must_use]
    pub const fn inverse_color(&self) -> bool {
        self.inverse_color
    }

    pub(crate) fn update_schema(&mut self, i: usize, fill: Color32) {
        match i {
            0 => self.app_background = fill,
            1 => self.subtle_background = fill,
            2 => self.ui_element_background = fill,
            3 => self.hovered_ui_element_background = fill,
            4 => self.active_ui_element_background = fill,
            5 => self.subtle_borders_and_separators = fill,
            6 => self.ui_element_border_and_focus_rings = fill,
            7 => self.hovered_ui_element_border = fill,
            8 => self.solid_backgrounds = fill,
            9 => self.hovered_solid_backgrounds = fill,
            10 => self.low_contrast_text = fill,
            11 => self.high_contrast_text = fill,
            _ => {}
        }
    }
    pub(crate) const fn get_token(&self, i: usize) -> Color32 {
        match i {
            0 => self.app_background,
            1 => self.subtle_background,
            2 => self.ui_element_background,
            3 => self.hovered_ui_element_background,
            4 => self.active_ui_element_background,
            5 => self.subtle_borders_and_separators,
            6 => self.ui_element_border_and_focus_rings,
            7 => self.hovered_ui_element_border,
            8 => self.solid_backgrounds,
            9 => self.hovered_solid_backgrounds,
            10 => self.low_contrast_text,
            11 => self.high_contrast_text,
            _ => Color32::TRANSPARENT,
        }
    }

    pub(crate) fn set_egui_visuals(&self, ctx: &egui::Context) {
        if ctx.style().visuals.dark_mode {
            ctx.set_visuals_of(egui::Theme::Dark, egui::Visuals::dark());
        } else {
            ctx.set_visuals_of(egui::Theme::Light, egui::Visuals::light());
        }
        let selection = egui::style::Selection {
            bg_fill: self.solid_backgrounds,
            stroke: Stroke::new(1.0, self.text_color),
        };
        let text_cursor = TextCursorStyle {
            stroke: Stroke::new(2.0, self.low_contrast_text),
            ..Default::default()
        };
        let widgets = egui::style::Widgets {
            noninteractive: WidgetVisuals {
                weak_bg_fill: self.subtle_background,
                bg_fill: self.subtle_background,
                bg_stroke: Stroke::new(1.0, self.subtle_borders_and_separators), // separators, indentation lines
                fg_stroke: Stroke::new(1.0, self.low_contrast_text), // normal text color
                rounding: Rounding::same(2.0),
                expansion: 0.0,
            },
            inactive: WidgetVisuals {
                weak_bg_fill: self.ui_element_background, // button background
                bg_fill: self.ui_element_background,      // checkbox background
                bg_stroke: Stroke::new(1.0, self.ui_element_background),
                fg_stroke: Stroke::new(1.0, self.low_contrast_text), // button text
                rounding: Rounding::same(2.0),
                expansion: 0.0,
            },
            hovered: WidgetVisuals {
                weak_bg_fill: self.hovered_ui_element_background,
                bg_fill: self.hovered_ui_element_background,
                bg_stroke: Stroke::new(1.0, self.hovered_ui_element_border), // e.g. hover over window edge or button
                fg_stroke: Stroke::new(1.5, self.high_contrast_text),
                rounding: Rounding::same(3.0),
                expansion: 1.0,
            },
            active: WidgetVisuals {
                weak_bg_fill: self.active_ui_element_background,
                bg_fill: self.active_ui_element_background,
                bg_stroke: Stroke::new(1.0, self.ui_element_border_and_focus_rings),
                fg_stroke: Stroke::new(2.0, self.high_contrast_text),
                rounding: Rounding::same(2.0),
                expansion: 1.0,
            },
            open: WidgetVisuals {
                weak_bg_fill: self.active_ui_element_background,
                bg_fill: self.active_ui_element_background,
                bg_stroke: Stroke::new(1.0, self.ui_element_border_and_focus_rings),
                fg_stroke: Stroke::new(1.0, self.high_contrast_text),
                rounding: Rounding::same(2.0),
                expansion: 0.0,
            },
        };

        ctx.style_mut(|style| {
            style.visuals.selection = selection;
            style.visuals.widgets = widgets;
            style.visuals.text_cursor = text_cursor;
            style.visuals.extreme_bg_color = self.app_background; // e.g. TextEdit background
            style.visuals.faint_bg_color = self.app_background; // striped grid is originally from_additive_luminance(5)
            style.visuals.code_bg_color = self.ui_element_background;
            style.visuals.window_fill = self.subtle_background;
            style.visuals.window_stroke = Stroke::new(1.0, self.subtle_borders_and_separators);
            style.visuals.panel_fill = self.subtle_background;
            style.visuals.hyperlink_color = self.hovered_solid_backgrounds;
            //style.visuals.override_text_color = Some(self.text_color());
        });
    }
}

/// A theme is basically a `[ColorPreset; 12]`.
///
/// # Examples
/// ```
/// use egui_colors::tokens::ColorPreset;
/// let mut my_theme = [ColorPreset::Indigo; 12];
/// my_theme[11] = ColorPreset::Custom([23, 45, 77]);
/// ```
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ColorPreset {
    #[default]
    Gray,
    EguiBlue,
    Tomato,
    Red,
    Ruby,
    Crimson,
    Pink,
    Plum,
    Purple,
    Violet,
    Iris,
    Indigo,
    Blue,
    Cyan,
    Teal,
    Jade,
    Green,
    Grass,
    Brown,
    Bronze,
    Gold,
    Orange,
    Custom([u8; 3]),
}

impl ColorPreset {
    pub(crate) fn get_srgb(self) -> LinSrgb<f32> {
        let [r, g, b] = self.rgb();
        Srgb::new(r, g, b).into_linear()
    }
    /// Returns the rgb values of this preset.
    ///
    /// Useful for example if you want to serialize the color theme.
    #[must_use]
    pub const fn rgb(&self) -> [u8; 3] {
        match *self {
            Self::Gray => [117, 117, 117],
            Self::EguiBlue => [0, 109, 143],
            Self::Tomato => [229, 77, 46],
            Self::Red => [229, 72, 77],
            Self::Ruby => [229, 70, 102],
            Self::Crimson => [233, 61, 130],
            Self::Pink => [214, 64, 159],
            Self::Plum => [171, 74, 186],
            Self::Purple => [142, 78, 198],
            Self::Violet => [110, 86, 207],
            Self::Iris => [91, 91, 214],
            Self::Indigo => [62, 99, 214],
            Self::Blue => [0, 144, 255],
            Self::Cyan => [0, 162, 199],
            Self::Teal => [18, 165, 148],
            Self::Jade => [41, 163, 131],
            Self::Green => [48, 164, 108],
            Self::Grass => [70, 167, 88],
            Self::Brown => [173, 127, 88],
            Self::Bronze => [161, 128, 114],
            Self::Gold => [151, 131, 101],
            Self::Orange => [247, 107, 21],
            Self::Custom([r, g, b]) => [r, g, b],
        }
    }
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Gray => "Gray",
            Self::EguiBlue => "EguiBlue",
            Self::Tomato => "Tomato",
            Self::Red => "Red",
            Self::Ruby => "Ruby",
            Self::Crimson => "Crimson",
            Self::Pink => "Pink",
            Self::Plum => "Plum",
            Self::Purple => "Purple",
            Self::Violet => "Violet",
            Self::Iris => "Iris",
            Self::Indigo => "Indigo",
            Self::Blue => "Blue",
            Self::Cyan => "Cyan",
            Self::Teal => "Teal",
            Self::Jade => "Jade",
            Self::Green => "Green",
            Self::Grass => "Grass",
            Self::Brown => "Brown",
            Self::Bronze => "Bronze",
            Self::Gold => "Gold",
            Self::Orange => "Orange",
            Self::Custom(_) => "Custom",
        }
    }
}
