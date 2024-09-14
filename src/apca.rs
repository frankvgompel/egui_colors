use eframe::egui;

// originals
const S_TRC: f32 = 2.4;
const N_TX: f32 = 0.57;
const N_BG: f32 = 0.56;
const R_TX: f32 = 0.62;
const R_BG: f32 = 0.65;
// clamps &ramp; Scalers
const B_CLIP: f32 = 1.414;
const B_THRSH: f32 = 0.022;
const W_SCALE: f32 = 1.14;
const W_OFFSET: f32 = 0.027;
const W_CLAMP: f32 = 0.1;

// sRGB coefficients
const S_RCO: f32 = 0.2126729;
const S_GCO: f32 = 0.7151522;
const S_BCO: f32 = 0.0721750;

fn fsc(y: f32) -> f32 {
    if y < B_THRSH {
        y + (B_THRSH - y).powf(B_CLIP)
    } else if y < 0.0 {
        0.0
    } else {
        y
    }
}

fn determine_cw(y_txt: f32, y_bg: f32) -> f32 {
    // normal polarity (dark text; light bg)
    if y_bg > y_txt {
        (y_bg.powf(N_BG) - y_txt.powf(N_TX)) * W_SCALE
    }
    // reverse polarity
    else {
        (y_bg.powf(R_BG) - y_txt.powf(R_TX)) * W_SCALE
    }
}

fn clamp_contrast(cw: f32) -> f32 {
    if cw.abs() < W_CLAMP {
        0.0
    } else if cw > 0.0 {
        cw - W_OFFSET
    } else {
        cw + W_OFFSET
    }
}

pub(crate) fn estimate_lc(rgb_txt: egui::Color32, rgb_bg: egui::Color32) -> f32 {
    let r = (rgb_txt.r() as f32 / 255.0).powf(S_TRC) * S_RCO;
    let g = (rgb_txt.g() as f32 / 255.0).powf(S_TRC) * S_GCO;
    let b = (rgb_txt.b() as f32 / 255.0).powf(S_TRC) * S_BCO;

    let y = r + g + b;

    let r2 = (rgb_bg.r() as f32 / 255.0).powf(S_TRC) * 0.2126729;
    let g2 = (rgb_bg.g() as f32 / 255.0).powf(S_TRC) * 0.7151522;
    let b2 = (rgb_bg.b() as f32 / 255.0).powf(S_TRC) * 0.0721750;

    let y2 = r2 + g2 + b2;

    let y_txt = fsc(y);
    let y_bg = fsc(y2);

    let cw = determine_cw(y_txt, y_bg);
    let s_apc = clamp_contrast(cw);
    s_apc * 100.0
}
