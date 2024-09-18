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
const S_RCO: f32 = 0.212_672_9;
const S_GCO: f32 = 0.715_152_2;
const S_BCO: f32 = 0.072_175_0;

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

pub fn estimate_lc(rgb_txt: egui::Color32, rgb_bg: egui::Color32) -> f32 {
    let r = (f32::from(rgb_txt.r()) / 255.0).powf(S_TRC) * S_RCO;
    let g = (f32::from(rgb_txt.g()) / 255.0).powf(S_TRC) * S_GCO;
    let b = (f32::from(rgb_txt.b()) / 255.0).powf(S_TRC) * S_BCO;

    let y = r + g + b;

    let r2 = (f32::from(rgb_bg.r()) / 255.0).powf(S_TRC) * 0.212_672_9;
    let g2 = (f32::from(rgb_bg.g()) / 255.0).powf(S_TRC) * 0.715_152_2;
    let b2 = (f32::from(rgb_bg.b()) / 255.0).powf(S_TRC) * 0.072_175_0;

    let y2 = r2 + g2 + b2;

    let y_txt = fsc(y);
    let y_bg = fsc(y2);

    let cw = determine_cw(y_txt, y_bg);
    let s_apc = clamp_contrast(cw);
    s_apc * 100.0
}
