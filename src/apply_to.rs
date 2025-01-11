use crate::tokens::ColorTokens;
use egui::Ui;

/// The `ApplyTo` enum is used to determine if a theme change should be applied
/// globally, locally or not at all.
#[derive(Debug, Copy, Clone)]
pub enum ApplyTo {
    Global,
    Local,
    Nothing,
}

impl ApplyTo {
    pub(crate) fn apply(self, ui: &mut Ui, tokens: &ColorTokens) {
        match self {
            Self::Global => {
                tokens.set_global_egui_visuals(ui.ctx());
            }
            Self::Local => {
                tokens.set_local_egui_visuals(ui);
            }
            Self::Nothing => {}
        }
    }
}
