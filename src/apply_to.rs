use egui::{Ui, Visuals};
use crate::Colorix;

/// The `ApplyTo` enum is used to determine if a theme change should be applied
/// globally, locally or not at all.
#[derive(Debug, Copy, Clone)]
pub enum ApplyTo {
    /// Theme changes are applied to the global egui context.
    Global,
    /// Theme changes are applied to the local widget tree only.
    Local,
    /// No theme changes are applied.
    ///
    /// This is useful when theme is applied manually in a different part of
    /// the interface.
    Nothing,
}

impl ApplyTo {
    pub(crate) fn apply(self, ui: &mut Ui, colorix: &Colorix) {
        match self {
            Self::Global => {
                colorix.apply_global(ui.ctx());
            }
            Self::Local => {
                colorix.apply_local(ui);
            }
            Self::Nothing => {}
        }
    }

    pub(crate) fn set_visuals(self, ui: &mut Ui, visuals: Visuals) {
        match self {
            Self::Global => {
                ui.ctx().set_visuals(visuals);
            }
            Self::Local => {
                ui.style_mut().visuals = visuals;
            }
            Self::Nothing => {}
        }
    }
}
