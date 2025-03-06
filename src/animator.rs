#![allow(clippy::too_many_lines)]
#![allow(clippy::semicolon_if_nothing_returned)]
#![allow(clippy::float_cmp)]

use crate::{tokens::ColorTokens, ApplyTo};
use egui::{
    style::{TextCursorStyle, WidgetVisuals},
    Color32, Context, CornerRadius, Id, Stroke, Style, Ui,
};

#[allow(clippy::many_single_char_names)]
fn interpolate_color(start: Color32, end: Color32, interpolation: f32) -> Color32 {
    let r = egui::lerp(f32::from(start.r())..=f32::from(end.r()), interpolation) as u8;
    let g = egui::lerp(f32::from(start.g())..=f32::from(end.g()), interpolation) as u8;
    let b = egui::lerp(f32::from(start.b())..=f32::from(end.b()), interpolation) as u8;
    let a = egui::lerp(f32::from(start.a())..=f32::from(end.a()), interpolation) as u8;
    Color32::from_rgba_premultiplied(r, g, b, a)
}

#[derive(Debug, Default, Clone)]
pub struct ColorAnimator {
    pub(crate) anim_id: Option<Id>,
    pub progress: f32,
    animation_time: f32,
    token_shift: bool,
    pub(crate) animation_done: bool,
    animation_in_progress: bool,
    values_1: [Color32; 3],
    values_2: [Color32; 3],
    pub tokenshifts: [Color32; 3],
    tokens: ColorTokens,
    pub animated_tokens: ColorTokens,
    shadow: Color32,
    s1: Color32,
    s2: Color32,
    pub(crate) apply_to: ApplyTo,
}

impl ColorAnimator {
    pub(crate) const fn new(tokens: ColorTokens) -> Self {
        Self {
            anim_id: None,
            progress: 0.0,
            animation_time: 1.,
            token_shift: true,
            animation_done: true,
            animation_in_progress: false,
            values_1: [Color32::TRANSPARENT; 3],
            values_2: [Color32::TRANSPARENT; 3],
            tokenshifts: [Color32::TRANSPARENT; 3],
            tokens,
            animated_tokens: tokens,
            shadow: Color32::TRANSPARENT,
            s1: Color32::from_black_alpha(25),
            s2: Color32::from_black_alpha(96),
            apply_to: ApplyTo::Global,
        }
    }

    pub(crate) fn set_animate(
        &mut self,
        ctx: Option<&Context>,
        ui: Option<&mut Ui>,
        tokens: ColorTokens,
    ) {
        if let Some(ctx) = ctx {
            if self.anim_id.is_none() {
                self.create_id(ctx);
            } else {
                self.animate(Some(ctx), None, tokens);
            }
        } else if let Some(ui) = ui {
            if self.anim_id.is_none() {
                let anim_id = ui.id();
                self.anim_id = Some(anim_id);
                ui.ctx().animate_value_with_time(anim_id, 0.0, 0.0);
            } else {
                self.animate(None, Some(ui), tokens);
            }
        }
    }
    pub(crate) const fn set_time(&mut self, new_time: f32) {
        self.animation_time = new_time;
    }
    pub(crate) fn create_id(&mut self, ctx: &Context) {
        let anim_id = egui::Id::new("Color animator");
        ctx.animate_value_with_time(anim_id, 0.0, 0.0);
        self.anim_id = Some(anim_id);
    }
    pub(crate) fn start(&mut self, ctx: &Context) {
        self.animation_done = false;
        if self.animation_in_progress {
            self.restart(ctx);
        } else {
            self.animation_done = false;
            self.animation_in_progress = true;
        }
    }
    // if animation in progress and needs a restart
    pub fn restart(&mut self, ctx: &Context) {
        self.animation_done = true;
        self.animation_in_progress = false;
        self.tokens = self.animated_tokens;
        if let Some(anim_id) = self.anim_id {
            ctx.animate_value_with_time(anim_id, 0.0, 0.0);
        }
        self.token_shift = !self.token_shift;
        self.start(ctx);
    }

    pub(crate) fn animate(
        &mut self,
        ctx: Option<&Context>,
        ui: Option<&mut Ui>,
        tokens: ColorTokens,
    ) {
        if self.animation_done {
            match self.apply_to {
                #[allow(clippy::needless_return)]
                ApplyTo::Global | ApplyTo::ExtraScale => return,
                // if animation done, local ui needs to keep updating every frame
                ApplyTo::Local => {
                    if let Some(ui) = ui {
                        self.apply_local_ui(ui.style_mut());
                    }
                }
            }
        } else {
            let Some(anim_id) = self.anim_id else { return };

            if let Some(ctx) = ctx {
                let shadow = if ctx.style().visuals.dark_mode {
                    self.s2
                } else {
                    self.s1
                };
                self.progress = ctx.animate_value_with_time(anim_id, 1.0, self.animation_time);
                ctx.style_mut(|style| self.set_egui_animation(style, tokens, shadow));
                if self.progress == 1.0 {
                    ctx.animate_value_with_time(anim_id, 0.0, 0.0);
                }
            } else if let Some(ui) = ui {
                let shadow = if ui.style_mut().visuals.dark_mode {
                    self.s2
                } else {
                    self.s1
                };
                self.progress = ui
                    .ctx()
                    .animate_value_with_time(anim_id, 1.0, self.animation_time);
                self.set_egui_animation(ui.style_mut(), tokens, shadow);
                if self.progress == 1.0 {
                    ui.ctx().animate_value_with_time(anim_id, 0.0, 0.0);
                }
            }
        }
    }
    fn set_egui_animation(&mut self, style: &mut Style, tokens: ColorTokens, shadow: Color32) {
        let indices = [[6, 0, 7], [8, 8, 6]];

        self.values_1
            .iter_mut()
            .enumerate()
            .for_each(|(i, v)| *v = self.animated_tokens.get_token(indices[0][i]));
        self.values_2
            .iter_mut()
            .enumerate()
            .for_each(|(i, v)| *v = self.animated_tokens.get_token(indices[1][i]));

        let (start_values, end_values) = if self.token_shift {
            (&self.values_1, &self.values_2)
        } else {
            (&self.values_2, &self.values_1)
        };
        self.tokenshifts.iter_mut().enumerate().for_each(|(i, v)| {
            *v = interpolate_color(start_values[i], end_values[i], self.progress)
        });

        self.animated_tokens.app_background = interpolate_color(
            self.tokens.app_background,
            tokens.app_background,
            self.progress,
        );
        self.animated_tokens.subtle_background = interpolate_color(
            self.tokens.subtle_background,
            tokens.subtle_background,
            self.progress,
        );
        self.animated_tokens.ui_element_background = interpolate_color(
            self.tokens.ui_element_background,
            tokens.ui_element_background,
            self.progress,
        );
        self.animated_tokens.hovered_ui_element_background = interpolate_color(
            self.tokens.hovered_ui_element_background,
            tokens.hovered_ui_element_background,
            self.progress,
        );
        self.animated_tokens.active_ui_element_background = interpolate_color(
            self.tokens.active_ui_element_background,
            tokens.active_ui_element_background,
            self.progress,
        );
        self.animated_tokens.subtle_borders_and_separators = interpolate_color(
            self.tokens.subtle_borders_and_separators,
            tokens.subtle_borders_and_separators,
            self.progress,
        );
        self.animated_tokens.ui_element_border_and_focus_rings = interpolate_color(
            self.tokens.ui_element_border_and_focus_rings,
            tokens.ui_element_border_and_focus_rings,
            self.progress,
        );
        self.animated_tokens.hovered_ui_element_border = interpolate_color(
            self.tokens.hovered_ui_element_border,
            tokens.hovered_ui_element_border,
            self.progress,
        );
        self.animated_tokens.solid_backgrounds = interpolate_color(
            self.tokens.solid_backgrounds,
            tokens.solid_backgrounds,
            self.progress,
        );
        self.animated_tokens.hovered_solid_backgrounds = interpolate_color(
            self.tokens.hovered_solid_backgrounds,
            tokens.hovered_solid_backgrounds,
            self.progress,
        );
        self.animated_tokens.low_contrast_text = interpolate_color(
            self.tokens.low_contrast_text,
            tokens.low_contrast_text,
            self.progress,
        );
        self.animated_tokens.high_contrast_text = interpolate_color(
            self.tokens.high_contrast_text,
            tokens.high_contrast_text,
            self.progress,
        );
        self.animated_tokens.on_accent =
            interpolate_color(self.tokens.on_accent, tokens.on_accent, self.progress);
        self.shadow = interpolate_color(self.shadow, shadow, self.progress);

        match self.apply_to {
            ApplyTo::Global | ApplyTo::Local => {
                let selection = egui::style::Selection {
                    bg_fill: self.animated_tokens.solid_backgrounds,
                    stroke: Stroke::new(1.0, self.animated_tokens.on_accent),
                };
                let text_cursor = TextCursorStyle {
                    stroke: Stroke::new(2.0, self.animated_tokens.low_contrast_text),
                    ..Default::default()
                };
                let widgets = egui::style::Widgets {
                    noninteractive: WidgetVisuals {
                        weak_bg_fill: self.animated_tokens.subtle_background,
                        bg_fill: self.animated_tokens.subtle_background,
                        bg_stroke: Stroke::new(
                            1.0,
                            self.animated_tokens.subtle_borders_and_separators,
                        ), // separators, indentation lines
                        fg_stroke: Stroke::new(1.0, self.animated_tokens.low_contrast_text), // normal text color
                        corner_radius: CornerRadius::same(2),
                        expansion: 0.0,
                    },
                    inactive: WidgetVisuals {
                        weak_bg_fill: self.animated_tokens.ui_element_background, // button background
                        bg_fill: self.animated_tokens.ui_element_background, // checkbox background
                        bg_stroke: Stroke::new(1.0, self.animated_tokens.ui_element_background),
                        fg_stroke: Stroke::new(1.0, self.animated_tokens.low_contrast_text), // button text
                        corner_radius: CornerRadius::same(2),
                        expansion: 0.0,
                    },
                    hovered: WidgetVisuals {
                        weak_bg_fill: self.animated_tokens.hovered_ui_element_background,
                        bg_fill: self.animated_tokens.hovered_ui_element_background,
                        bg_stroke: Stroke::new(1.0, self.animated_tokens.hovered_ui_element_border), // e.g. hover over window edge or button
                        fg_stroke: Stroke::new(1.5, self.animated_tokens.high_contrast_text),
                        corner_radius: CornerRadius::same(3),
                        expansion: 1.0,
                    },
                    active: WidgetVisuals {
                        weak_bg_fill: self.animated_tokens.active_ui_element_background,
                        bg_fill: self.animated_tokens.active_ui_element_background,
                        bg_stroke: Stroke::new(
                            1.0,
                            self.animated_tokens.ui_element_border_and_focus_rings,
                        ),
                        fg_stroke: Stroke::new(2.0, self.animated_tokens.high_contrast_text),
                        corner_radius: CornerRadius::same(2),
                        expansion: 1.0,
                    },
                    open: WidgetVisuals {
                        weak_bg_fill: self.animated_tokens.active_ui_element_background,
                        bg_fill: self.animated_tokens.active_ui_element_background,
                        bg_stroke: Stroke::new(
                            1.0,
                            self.animated_tokens.ui_element_border_and_focus_rings,
                        ),
                        fg_stroke: Stroke::new(1.0, self.animated_tokens.high_contrast_text),
                        corner_radius: CornerRadius::same(2),
                        expansion: 0.0,
                    },
                };
                style.visuals.selection = selection;
                style.visuals.widgets = widgets;
                style.visuals.text_cursor = text_cursor;
                style.visuals.extreme_bg_color = self.animated_tokens.app_background; // e.g. TextEdit background
                style.visuals.faint_bg_color = self.animated_tokens.app_background; // striped grid is originally from_additive_luminance(5)
                style.visuals.code_bg_color = self.animated_tokens.ui_element_background;
                style.visuals.window_fill = self.animated_tokens.subtle_background;
                style.visuals.window_stroke =
                    Stroke::new(1.0, self.animated_tokens.subtle_borders_and_separators);
                style.visuals.panel_fill = self.animated_tokens.subtle_background;
                style.visuals.hyperlink_color = self.animated_tokens.hovered_solid_backgrounds;
                style.visuals.window_shadow.color = self.shadow;

                // reset old values and flag of animate value
                if self.progress == 1.0 {
                    self.tokens = self.animated_tokens;
                    self.animation_done = true;
                    self.animation_in_progress = false;
                    self.token_shift = !self.token_shift;
                }
            }
            ApplyTo::ExtraScale => {
                if self.progress == 1.0 {
                    self.tokens = self.animated_tokens;
                    self.animation_done = true;
                    self.animation_in_progress = false;
                    self.token_shift = !self.token_shift;
                }
            }
        }
    }
    fn apply_local_ui(&self, style: &mut egui::style::Style) {
        let selection = egui::style::Selection {
            bg_fill: self.animated_tokens.solid_backgrounds,
            stroke: Stroke::new(1.0, self.animated_tokens.on_accent),
        };
        let text_cursor = TextCursorStyle {
            stroke: Stroke::new(2.0, self.animated_tokens.low_contrast_text),
            ..Default::default()
        };
        let widgets = egui::style::Widgets {
            noninteractive: WidgetVisuals {
                weak_bg_fill: self.animated_tokens.subtle_background,
                bg_fill: self.animated_tokens.subtle_background,
                bg_stroke: Stroke::new(1.0, self.animated_tokens.subtle_borders_and_separators), // separators, indentation lines
                fg_stroke: Stroke::new(1.0, self.animated_tokens.low_contrast_text), // normal text color
                corner_radius: CornerRadius::same(2),
                expansion: 0.0,
            },
            inactive: WidgetVisuals {
                weak_bg_fill: self.animated_tokens.ui_element_background, // button background
                bg_fill: self.animated_tokens.ui_element_background,      // checkbox background
                bg_stroke: Stroke::new(1.0, self.animated_tokens.ui_element_background),
                fg_stroke: Stroke::new(1.0, self.animated_tokens.low_contrast_text), // button text
                corner_radius: CornerRadius::same(2),
                expansion: 0.0,
            },
            hovered: WidgetVisuals {
                weak_bg_fill: self.animated_tokens.hovered_ui_element_background,
                bg_fill: self.animated_tokens.hovered_ui_element_background,
                bg_stroke: Stroke::new(1.0, self.animated_tokens.hovered_ui_element_border), // e.g. hover over window edge or button
                fg_stroke: Stroke::new(1.5, self.animated_tokens.high_contrast_text),
                corner_radius: CornerRadius::same(3),
                expansion: 1.0,
            },
            active: WidgetVisuals {
                weak_bg_fill: self.animated_tokens.active_ui_element_background,
                bg_fill: self.animated_tokens.active_ui_element_background,
                bg_stroke: Stroke::new(1.0, self.animated_tokens.ui_element_border_and_focus_rings),
                fg_stroke: Stroke::new(2.0, self.animated_tokens.high_contrast_text),
                corner_radius: CornerRadius::same(2),
                expansion: 1.0,
            },
            open: WidgetVisuals {
                weak_bg_fill: self.animated_tokens.active_ui_element_background,
                bg_fill: self.animated_tokens.active_ui_element_background,
                bg_stroke: Stroke::new(1.0, self.animated_tokens.ui_element_border_and_focus_rings),
                fg_stroke: Stroke::new(1.0, self.animated_tokens.high_contrast_text),
                corner_radius: CornerRadius::same(2),
                expansion: 0.0,
            },
        };
        style.visuals.selection = selection;
        style.visuals.widgets = widgets;
        style.visuals.text_cursor = text_cursor;
        style.visuals.extreme_bg_color = self.animated_tokens.app_background; // e.g. TextEdit background
        style.visuals.faint_bg_color = self.animated_tokens.app_background; // striped grid is originally from_additive_luminance(5)
        style.visuals.code_bg_color = self.animated_tokens.ui_element_background;
        style.visuals.window_fill = self.animated_tokens.subtle_background;
        style.visuals.window_stroke =
            Stroke::new(1.0, self.animated_tokens.subtle_borders_and_separators);
        style.visuals.panel_fill = self.animated_tokens.subtle_background;
        style.visuals.hyperlink_color = self.animated_tokens.hovered_solid_backgrounds;
        style.visuals.window_shadow.color = self.shadow;
    }
}
