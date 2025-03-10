// OkHsl color space copyright by Björn Ottosson  https://bottosson.github.io/posts/colorpicker/
#![allow(clippy::suboptimal_flops)]
#![allow(clippy::use_self)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::similar_names)]
#![allow(clippy::useless_let_if_seq)]

use std::f32::consts::PI;

#[derive(Debug, Default, Clone, Copy)]
pub struct LinSrgb {
    red: f32,
    green: f32,
    blue: f32,
}

impl LinSrgb {
    pub const fn new(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue }
    }
    pub fn lighten(&self, factor: f32) -> Self {
        Self {
            red: (self.red + factor * (1. - self.red)).clamp(0., 1.),
            green: (self.green + factor * (1. - self.green)).clamp(0., 1.),
            blue: (self.blue + factor * (1. - self.blue)).clamp(0., 1.),
        }
    }
    pub fn to_okhsl(self) -> Okhsl {
        let oklab = linear_srgb_to_oklab(self);
        oklab_to_okhsl(oklab)
    }
    pub fn darken(&self, factor: f32) -> Self {
        Self {
            red: (self.red - factor * (self.red)).clamp(0., 1.),
            green: (self.green - factor * (self.green)).clamp(0., 1.),
            blue: (self.blue - factor * (self.blue)).clamp(0., 1.),
        }
    }
    #[allow(clippy::wrong_self_convention)]
    fn from_linear(&self) -> [u8; 3] {
        [
            gamma_u8_from_linear_f32(self.red),
            gamma_u8_from_linear_f32(self.green),
            gamma_u8_from_linear_f32(self.blue),
        ]
    }
    pub fn into_linear(rgb: [u8; 3]) -> Self {
        let r = linear_f32_from_gamma_u8(rgb[0]);
        let g = linear_f32_from_gamma_u8(rgb[1]);
        let b = linear_f32_from_gamma_u8(rgb[2]);
        Self::new(r, g, b)
    }
}

fn linear_f32_from_gamma_u8(s: u8) -> f32 {
    if s <= 10 {
        f32::from(s) / 3294.6
    } else {
        ((f32::from(s) + 14.025) / 269.025).powf(2.4)
    }
}

fn gamma_u8_from_linear_f32(l: f32) -> u8 {
    if l <= 0.0 {
        0
    } else if l <= 0.003_130_8 {
        fast_round(3294.6 * l)
    } else if l <= 1.0 {
        fast_round(269.025 * l.powf(1.0 / 2.4) - 14.025)
    } else {
        255
    }
}
fn fast_round(r: f32) -> u8 {
    (r + 0.5) as _
}
pub fn from_degrees(hue: f32) -> f32 {
    (hue / 360.).clamp(0., 1.)
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
struct Oklab {
    l: f32,
    a: f32,
    b: f32,
}

impl Oklab {
    fn to_linear_srgb(self) -> LinSrgb {
        let l_ = 0.215_803_76_f32.mul_add(self.b, 0.396_337_78_f32.mul_add(self.a, self.l));
        let m_ = (-0.063_854_17_f32).mul_add(self.b, (-0.105_561_346_f32).mul_add(self.a, self.l));
        let s_ = (-1.291_485_5_f32).mul_add(self.b, (-0.089_484_18_f32).mul_add(self.a, self.l));

        let l = l_ * l_ * l_;
        let m = m_ * m_ * m_;
        let s = s_ * s_ * s_;

        LinSrgb {
            red: 0.230_969_94_f32.mul_add(s, 4.076_741_7_f32.mul_add(l, -3.307_711_6 * m)),
            green: (-0.341_319_38_f32).mul_add(s, (-1.268_438_f32).mul_add(l, 2.609_757_4 * m)),
            blue: 1.707_614_7_f32.mul_add(s, (-0.004_196_086_3_f32).mul_add(l, -0.703_418_6 * m)),
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Okhsl {
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
}

impl Okhsl {
    fn to_oklab(self) -> Oklab {
        okhsl_to_oklab(self)
    }
    pub fn lighten(&self, factor: f32) -> Okhsl {
        Okhsl {
            hue: self.hue,
            saturation: self.saturation,
            lightness: (self.lightness + factor * (1. - self.lightness)).clamp(0., 1.),
        }
    }
    pub fn darken(&self, factor: f32) -> Okhsl {
        Okhsl {
            hue: self.hue,
            saturation: self.saturation,
            lightness: (self.lightness - factor * self.lightness).clamp(0., 1.),
        }
    }
    pub fn from_color(rgb: LinSrgb) -> Okhsl {
        rgb.to_okhsl()
    }
    pub fn as_degrees(&self) -> f32 {
        let hue = self.hue;
        (hue * 360.).clamp(0., 360.)
    }
    pub fn to_srgb(self) -> LinSrgb {
        let oklab = self.to_oklab();
        oklab.to_linear_srgb()
    }
    pub fn to_u8(self) -> [u8; 3] {
        let rgb = self.to_srgb();
        rgb.from_linear()
    }
}

fn linear_srgb_to_oklab(c: LinSrgb) -> Oklab {
    let l = 0.051_445_995_f32.mul_add(
        c.blue,
        0.412_221_46_f32.mul_add(c.red, 0.536_332_55 * c.green),
    );
    let m = 0.107_396_96_f32.mul_add(
        c.blue,
        0.211_903_5_f32.mul_add(c.red, 0.680_699_5 * c.green),
    );
    let s = 0.629_978_7_f32.mul_add(
        c.blue,
        0.088_302_46_f32.mul_add(c.red, 0.281_718_85 * c.green),
    );

    let l_ = l.cbrt();
    let m_ = m.cbrt();
    let s_ = s.cbrt();

    Oklab {
        l: (-0.004_072_047_f32).mul_add(s_, 0.210_454_26_f32.mul_add(l_, 0.793_617_8 * m_)),
        a: 0.450_593_7_f32.mul_add(s_, 1.977_998_5_f32.mul_add(l_, -2.428_592_2 * m_)),
        b: (-0.808_675_77_f32).mul_add(s_, 0.025_904_037_f32.mul_add(l_, 0.782_771_77 * m_)),
    }
}
fn compute_max_saturation(a: f32, b: f32) -> f32 {
    let [k0, k1, k2, k3, k4, wl, wm, ws] =
        if (-1.881_703_3_f32).mul_add(a, -(0.809_364_9 * b)) > 1.0 {
            [
                1.190_862_8,
                1.765_767_3,
                0.596_626_4,
                0.755_152,
                0.567_712_4,
                4.076_741_7,
                -3.307_711_6,
                0.230_969_94,
            ]
        } else if 1.814_441_1_f32.mul_add(a, -(1.194_452_8 * b)) > 1.0 {
            [
                0.739_565_15,
                -0.459_544_04,
                0.082_854_27,
                0.125_410_7,
                0.145_032_04,
                -1.268_438,
                2.609_757_4,
                -0.341_319_38,
            ]
        } else {
            [
                1.357_336_5,
                -0.009_157_99,
                -1.151_302_1,
                -0.505_596_06,
                0.006_921_67,
                -0.004_196_086_3,
                -0.703_418_6,
                1.707_614_7,
            ]
        };
    let mut saturation = (k4 * a).mul_add(b, (k3 * a).mul_add(a, k2.mul_add(b, k1.mul_add(a, k0))));

    let k_l = 0.396_337_78_f32.mul_add(a, 0.215_803_76 * b);
    let k_m = (-0.105_561_346_f32).mul_add(a, -(0.063_854_17 * b));
    let k_s = (-0.089_484_18_f32).mul_add(a, -(1.291_485_5 * b));

    {
        let l_ = saturation.mul_add(k_l, 1.);
        let m_ = saturation.mul_add(k_m, 1.);
        let s_ = saturation.mul_add(k_s, 1.);

        let l = l_ * l_ * l_;
        let m = m_ * m_ * m_;
        let s = s_ * s_ * s_;

        let l_d_s = 3. * (k_l * l_) * l_;
        let m_d_s = 3. * (k_m * m_) * m_;
        let s_d_s = 3. * (k_s * s_) * s_;

        let l_d_s2 = 6. * k_l * (k_l * l_);
        let m_d_s2 = 6. * k_m * (k_m * m_);
        let s_d_s2 = 6. * k_s * (k_s * s_);

        let f = ws.mul_add(s, wl.mul_add(l, wm * m));
        let f1 = ws.mul_add(s_d_s, wl.mul_add(l_d_s, wm * m_d_s));
        debug_assert!(f1 != 0.);
        let f2 = ws.mul_add(s_d_s2, wl.mul_add(l_d_s2, wm * m_d_s2));

        let div = f1.mul_add(f1, -(0.5 * f * f2));
        saturation -= f * f1 / div;
    }
    saturation
}
fn find_cusp(a: f32, b: f32) -> [f32; 2] {
    let s_cusp = compute_max_saturation(a, b);
    let l_cusp = scale_l(1.0, s_cusp, a, b);
    [l_cusp, (l_cusp * s_cusp)]
}
fn find_gamut_intersection(
    a: f32,
    b: f32,
    l_1: f32,
    c_1: f32,
    l_0: f32,
    cusp: Option<[f32; 2]>,
) -> f32 {
    let [cusp_l, cusp_c] = cusp.unwrap_or_else(|| find_cusp(a, b));
    let mut t;
    if (l_1 - l_0).mul_add(cusp_c, -((cusp_l - l_0) * c_1)) <= 0.0 {
        t = cusp_c * l_0 / c_1.mul_add(cusp_l, cusp_c * (l_0 - l_1));
    } else {
        t = cusp_c * (l_0 - 1.0) / c_1.mul_add(cusp_l - 1.0, cusp_c * (l_0 - l_1));
        {
            let d_l = l_1 - l_0;
            let d_c = c_1;

            let k_l = 0.3963_3778_f32.mul_add(a, 0.2158_0376 * b);
            let k_m = (-0.1055_6134_6_f32).mul_add(a, -(0.0638_5417 * b));
            let k_s = (-0.0894_8418_f32).mul_add(a, -(1.2914_855 * b));

            let l_dt = d_c.mul_add(k_l, d_l);
            let m_dt = d_c.mul_add(k_m, d_l);
            let s_dt = d_c.mul_add(k_s, d_l);
            {
                let l = l_0.mul_add(1. - t, t * l_1);
                let c = t * c_1;

                let l_ = c.mul_add(k_l, l);
                let m_ = c.mul_add(k_m, l);
                let s_ = c.mul_add(k_s, l);

                let l = l_ * l_ * l_;
                let m = m_ * m_ * m_;
                let s = s_ * s_ * s_;

                let ldt = 3. * (l_dt * l_) * l_;
                let mdt = 3. * (m_dt * m_) * m_;
                let sdt = 3. * (s_dt * s_) * s_;

                let ldt2 = 6. * l_dt * (l_dt * l_);
                let mdt2 = 6. * m_dt * (m_dt * m_);
                let sdt2 = 6. * s_dt * (s_dt * s_);

                let r =
                    0.2309_6994_f32.mul_add(s, 4.0767_417_f32.mul_add(l, -(3.3077_116 * m))) - 1.;
                let r1 =
                    0.2309_6994_f32.mul_add(sdt, 4.0767_417_f32.mul_add(ldt, -(3.3077_116 * mdt)));
                let r2 = 0.2309_6994_f32
                    .mul_add(sdt2, 4.0767_417_f32.mul_add(ldt2, -(3.3077_116 * mdt2)));

                let u_r = r1 / r1.mul_add(r1, -(0.5 * r * r2));
                let t_r = -r * u_r;

                let g =
                    0.3413_1938_f32.mul_add(-s, (-1.2684_38_f32).mul_add(l, 2.6097_574 * m)) - 1.;
                let g1 =
                    0.3413_1938_f32.mul_add(-sdt, (-1.2684_38_f32).mul_add(ldt, 2.6097_574 * mdt));
                let g2 = 0.3413_1938_f32
                    .mul_add(-sdt2, (-1.268_438_f32).mul_add(ldt2, 2.6097_574 * mdt2));

                let u_g = g1 / g1.mul_add(g1, -(0.5 * g * g2));
                let t_g = -g * u_g;

                let b = 1.7076_147_f32
                    .mul_add(s, (-0.0041_9608_63_f32).mul_add(l, -(0.7034_186 * m)))
                    - 1.;
                let b1 = 1.7076_147_f32
                    .mul_add(sdt, (-0.0041_9608_63_f32).mul_add(ldt, -(0.7034_186 * mdt)));
                let b2 = 1.7076_147_f32.mul_add(
                    sdt2,
                    (-0.0041_9608_63_f32).mul_add(ldt2, -(0.7034_186 * mdt2)),
                );

                let u_b = b1 / b1.mul_add(b1, -(0.5 * b * b2));
                let t_b = -b * u_b;

                let t_r = if u_r >= 0.0 { t_r } else { 10e5 };
                let t_g = if u_g >= 0.0 { t_g } else { 10e5 };
                let t_b = if u_b >= 0.0 { t_b } else { 10e5 };

                t += t_r.min(t_g.min(t_b));
            }
        }
    }
    t
}
fn toe(x: f32) -> f32 {
    let k_1: f32 = 0.206;
    let k_2: f32 = 0.03;
    let k_3: f32 = (1. + k_1) / (1. + k_2);
    0.5 * (k_3.mul_add(x, -k_1)
        + k_3
            .mul_add(x, -k_1)
            .mul_add(k_3.mul_add(x, -k_1), 4. * k_2 * (k_3 * x))
            .sqrt())
}
fn toe_inv(x: f32) -> f32 {
    let k_1 = 0.206;
    let k_2 = 0.03;
    let k_3 = (1. + k_1) / (1. + k_2);
    x.mul_add(x, k_1 * x) / (k_3 * (x + k_2))
}
fn st_mid(a_: f32, b_: f32) -> [f32; 2] {
    debug_assert!(a_.is_finite());
    debug_assert!(b_.is_finite());
    let s_mid = 0.1151_6993
        + 1. / a_.mul_add(
            a_.mul_add(
                a_.mul_add(
                    4.69891_f32.mul_add(a_, 5.3877_08_f32.mul_add(b_, -4.2489_457)),
                    10.02301_f32.mul_add(-b_, -2.1370_494),
                ),
                1.751_984_f32.mul_add(b_, -2.1955_736),
            ),
            4.1590_123_f32.mul_add(b_, 7.4477_897),
        );

    let t_mid = 0.1123_9642
        + 1. / a_.mul_add(
            a_.mul_add(
                a_.mul_add(
                    0.146_618_72_f32.mul_add(-a_, 0.453_995_68_f32.mul_add(-b_, 0.002_992_15)),
                    0.612_239_9_f32.mul_add(b_, -0.270_879_43),
                ),
                0.901_481_2_f32.mul_add(b_, 0.403_706_12),
            ),
            0.681_243_8_f32.mul_add(-b_, 1.613_203_2),
        );
    [s_mid, t_mid]
}
fn st_max(a_: f32, b_: f32, cusp: Option<[f32; 2]>) -> [f32; 2] {
    let [l, c] = cusp.unwrap_or_else(|| find_cusp(a_, b_));
    [c / l, c / (1. - l)]
}
fn get_cs(l: f32, a_: f32, b_: f32) -> [f32; 3] {
    let cusp = find_cusp(a_, b_);
    let c_max = find_gamut_intersection(a_, b_, l, 1.0, l, Some(cusp));
    let [s_max, t_max] = st_max(a_, b_, Some(cusp));
    let [s_mid, t_mid] = st_mid(a_, b_);

    let k = c_max / (l * s_max).min((1. - l) * t_max);
    let c_mid = {
        let c_a = l * s_mid;
        let c_b = (1. - l) * t_mid;
        let ca4 = (c_a * c_a) * (c_a * c_a);
        let cb4 = (c_b * c_b) * (c_b * c_b);

        0.9 * k * ((1. / (1. / ca4 + 1. / cb4)).sqrt()).sqrt()
    };
    let c_0 = {
        let c_a = l * 0.4;
        let c_b = (1. - l) * 0.8;

        (1. / (1. / (c_a * c_a) + 1. / (c_b * c_b))).sqrt()
    };
    [c_0, c_mid, c_max]
}
fn okhsl_to_oklab(
    Okhsl {
        hue: h,
        saturation: s,
        lightness: l,
    }: Okhsl,
) -> Oklab {
    if l <= 0. || l >= 1. {
        return Oklab { l, a: 0., b: 0. };
    }
    let a_ = (2. * PI * h).cos();
    let b_ = (2. * PI * h).sin();
    let l = toe_inv(l);

    let [c_0, c_mid, c_max] = get_cs(l, a_, b_);
    let t;
    let k_0;
    let k_1;
    let k_2;
    if s < 0.8 {
        t = 1.25 * s;
        k_0 = 0.;
        k_1 = 0.8 * c_0;
        k_2 = 1. - k_1 / c_mid;
    } else {
        t = 5. * (s - 0.8);
        k_0 = c_mid;
        k_1 = 0.2 * (c_mid * c_mid) * (1.25 * 1.25) / c_0;
        k_2 = 1. - k_1 / (c_max - c_mid);
    }
    let c = k_0 + t * k_1 / k_2.mul_add(-t, 1.);
    Oklab {
        l,
        a: c * a_,
        b: c * b_,
    }
}
fn oklab_to_okhsl(Oklab { l, a, b }: Oklab) -> Okhsl {
    if !(l > 0. && l < 1. && (a != 0. || b != 0.)) {
        return Okhsl {
            hue: 0.,
            saturation: 0.,
            lightness: l,
        };
    }
    let (h, a_, b_, c) = hue(b, a);
    let [c_0, c_mid, c_max] = get_cs(l, a_, b_);
    let s = if c < c_mid {
        let k_0 = 0.;
        let k_1 = 0.8 * c_0;
        let k_2 = 1. - k_1 / c_mid;
        let t = (c - k_0) / k_2.mul_add(c - k_0, k_1);
        t * 0.8
    } else {
        let k_0 = c_mid;
        let k_1 = 0.2 * (c_mid * c_mid) * (1.25 * 1.25) / c_0;
        let k_2 = 1. - k_1 / (c_max - c_mid);
        let t = (c - k_0) / k_2.mul_add(c - k_0, k_1);
        0.2_f32.mul_add(t, 0.8)
    };
    Okhsl {
        hue: h,
        saturation: s,
        lightness: toe(l),
    }
}
fn hue(b: f32, a: f32) -> (f32, f32, f32, f32) {
    let h = (0.5 * (-b).atan2(-a)).mul_add(1. / PI, 0.5);
    let c = a.hypot(b);
    let a_ = a * (1. / c);
    let b_ = b * (1. / c);
    (h, a_, b_, c)
}
fn scale_l(l_vt: f32, c_vt: f32, a_: f32, b_: f32) -> f32 {
    let rgb_scale = (Oklab {
        l: l_vt,
        a: a_ * c_vt,
        b: b_ * c_vt,
    })
    .to_linear_srgb();
    let rgb_max = rgb_scale
        .red
        .max(rgb_scale.green)
        .max(rgb_scale.blue.max(0.));
    (1. / rgb_max).cbrt()
}
