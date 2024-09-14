use eframe::egui::{self, style::{TextCursorStyle, WidgetVisuals}, Color32, Rounding, Stroke};
use crate::apca::estimate_lc;

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
            self.text_color = hsva.into()  
        }
        else {
            self.text_color = egui::Color32::WHITE
        }
    }

    /// the color for the text on accented color (solid backgrounds)
    pub fn text_color(&self) -> Color32 {
        self.text_color
    }

    /// notifies when lc > -46. 
    pub fn inverse_color(&self) -> bool {
        self. inverse_color
    }

    pub(crate) fn update_schema(&mut self, i: usize, fill: Color32) {
        match i {
            0 => { self.app_background = fill }
            1 => { self.subtle_background = fill }
            2 => { self.ui_element_background = fill }
            3 => { self.hovered_ui_element_background = fill }
            4 => { self.active_ui_element_background = fill }
            5 => { self.subtle_borders_and_separators = fill }
            6 => { self.ui_element_border_and_focus_rings = fill }
            7 => { self.hovered_ui_element_border = fill }
            8 => { self.solid_backgrounds = fill }
            9 => { self.hovered_solid_backgrounds = fill }
            10 => { self.low_contrast_text = fill }
            11 => { self.high_contrast_text = fill }
            _ => {}
        }
    }

    pub(crate) fn set_egui_visuals(&self, ctx: &egui::Context) {
        if ctx.style().visuals.dark_mode {
            ctx.set_visuals(egui::Visuals::dark())
        }
        else {
            ctx.set_visuals(egui::Visuals::light())
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
            style.visuals.extreme_bg_color = self.app_background;   // e.g. TextEdit background
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

/// A theme is basically a [ColorPreset; 12].
/// 
/// # Examples
/// ```
/// let mut my_theme = [ColorPreset::Indigo; 12]
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
  pub(crate) fn get_rgb(&self) -> [u8; 3] {
    match self {
        ColorPreset::Gray => [117, 117, 117],
        ColorPreset::EguiBlue => [0, 109, 143],
        ColorPreset::Tomato => [229, 77, 46],
        ColorPreset::Red => [229, 72, 77],
        ColorPreset::Ruby => [229, 70, 102],
        ColorPreset::Crimson => [233, 61, 130],
        ColorPreset::Pink => [214, 64, 159],
        ColorPreset::Plum => [171, 74, 186],
        ColorPreset::Purple => [142, 78, 198],
        ColorPreset::Violet => [110, 86, 207],
        ColorPreset::Iris => [91, 91, 214],
        ColorPreset::Indigo => [62, 99, 214],
        ColorPreset::Blue => [0, 144, 255],
        ColorPreset::Cyan => [0, 162, 199],
        ColorPreset::Teal => [18, 165, 148],
        ColorPreset::Jade => [41, 163, 131],
        ColorPreset::Green => [48, 164, 108],
        ColorPreset::Grass => [70, 167, 88],
        ColorPreset::Brown => [173, 127, 88],
        ColorPreset::Bronze => [161, 128, 114],
        ColorPreset::Gold => [151, 131, 101],
        ColorPreset::Orange => [247, 107, 21],
        ColorPreset::Custom(v) => *v,
    }
  }
}