use crate::{tokens::ColorPreset, Theme};

pub const EGUI_THEME: Theme = [
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::EguiBlue,
    ColorPreset::EguiBlue,
    ColorPreset::Gray,
    ColorPreset::Gray,
];

pub const OFFICE_GRAY: Theme = [
    ColorPreset::Custom([140, 149, 138]),
    ColorPreset::Custom([140, 149, 138]),
    ColorPreset::Custom([140, 149, 138]),
    ColorPreset::Custom([122, 166, 168]),
    ColorPreset::Gray,
    ColorPreset::Custom([122, 166, 168]),
    ColorPreset::Custom([122, 166, 168]),
    ColorPreset::Custom([122, 166, 168]),
    ColorPreset::Custom([59, 71, 97]),
    ColorPreset::Custom([59, 71, 97]),
    ColorPreset::Custom([185, 178, 168]),
    ColorPreset::Custom([185, 178, 168]),
];

pub const INDIGO_JADE: Theme = [
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Indigo,
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Jade,
    ColorPreset::Jade,
    ColorPreset::Jade,
    ColorPreset::Gray,
    ColorPreset::Gray,
];

pub const GRASS_BRONZE: Theme = [
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Grass,
    ColorPreset::Bronze,
    ColorPreset::Bronze,
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Green,
    ColorPreset::Bronze,
    ColorPreset::Bronze,
    ColorPreset::Gray,
    ColorPreset::Gray,
];

pub const WARM: Theme = [
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Orange,
    ColorPreset::Gold,
    ColorPreset::Gold,
    ColorPreset::Gold,
    ColorPreset::Red,
    ColorPreset::Red,
    ColorPreset::Gold,
    ColorPreset::Gold,
    ColorPreset::Gray,
    ColorPreset::Teal,
];

pub const COOL: Theme = [
    ColorPreset::Gray,
    ColorPreset::Indigo,
    ColorPreset::Indigo,
    ColorPreset::Iris,
    ColorPreset::Indigo,
    ColorPreset::Gray,
    ColorPreset::Iris,
    ColorPreset::Indigo,
    ColorPreset::Blue,
    ColorPreset::Indigo,
    ColorPreset::Orange,
    ColorPreset::Gray,
];

pub const SEVENTIES: Theme = [
    ColorPreset::Custom([95, 78, 163]),
    ColorPreset::Pink,
    ColorPreset::Pink,
    ColorPreset::Custom([95, 78, 163]),
    ColorPreset::Custom([95, 78, 163]),
    ColorPreset::Custom([254, 180, 0]),
    ColorPreset::Custom([95, 78, 163]),
    ColorPreset::Custom([95, 78, 163]),
    ColorPreset::Custom([254, 180, 0]),
    ColorPreset::Custom([254, 180, 0]),
    ColorPreset::Gray,
    ColorPreset::Gray,
];

pub(crate) const THEMES: [Theme; 7] = [
    EGUI_THEME,
    INDIGO_JADE,
    GRASS_BRONZE,
    WARM,
    COOL,
    SEVENTIES,
    OFFICE_GRAY,
];

pub(crate) const LABELS: [&str; 12] = [
    "app background",
    "subtle background",
    "ui element background",
    "hovered ui element background",
    "active ui element background",
    "subtle borders and separators",
    "ui element border and focus rings",
    "hovered ui element border",
    "solid backgrounds",
    "hovered solid backgrounds",
    "low contrast text",
    "high contrast text",
];

pub(crate) const THEME_NAMES: [&str; 7] = [
    "Egui",
    "Indigo/jade",
    "Grass/bronze",
    "Warm",
    "Cool",
    "Seventies",
    "Office Gray",
];
