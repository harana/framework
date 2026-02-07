pub mod output;

use output::*;

/// Parse a hex color string to RGB components
fn parse_hex(hex: &str) -> Result<(u8, u8, u8), String> {
    let hex = hex.trim_start_matches('#');
    
    let (r, g, b) = match hex.len() {
        3 => {
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).map_err(|e| e.to_string())?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).map_err(|e| e.to_string())?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).map_err(|e| e.to_string())?;
            (r, g, b)
        }
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).map_err(|e| e.to_string())?;
            let g = u8::from_str_radix(&hex[2..4], 16).map_err(|e| e.to_string())?;
            let b = u8::from_str_radix(&hex[4..6], 16).map_err(|e| e.to_string())?;
            (r, g, b)
        }
        _ => return Err("Invalid hex color format".to_string()),
    };
    
    Ok((r, g, b))
}

/// Convert RGB to hex string
fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

/// Convert RGB to HSL
fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    let r = r as f64 / 255.0;
    let g = g as f64 / 255.0;
    let b = b as f64 / 255.0;
    
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;
    
    if max == min {
        return (0.0, 0.0, l);
    }
    
    let d = max - min;
    let s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };
    
    let h = if max == r {
        ((g - b) / d + if g < b { 6.0 } else { 0.0 }) / 6.0
    } else if max == g {
        ((b - r) / d + 2.0) / 6.0
    } else {
        ((r - g) / d + 4.0) / 6.0
    };
    
    (h * 360.0, s, l)
}

/// Convert HSL to RGB
fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    if s == 0.0 {
        let v = (l * 255.0).round() as u8;
        return (v, v, v);
    }
    
    let h = h / 360.0;
    let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
    let p = 2.0 * l - q;
    
    let hue_to_rgb = |p: f64, q: f64, mut t: f64| -> f64 {
        if t < 0.0 { t += 1.0; }
        if t > 1.0 { t -= 1.0; }
        if t < 1.0/6.0 { return p + (q - p) * 6.0 * t; }
        if t < 1.0/2.0 { return q; }
        if t < 2.0/3.0 { return p + (q - p) * (2.0/3.0 - t) * 6.0; }
        p
    };
    
    let r = (hue_to_rgb(p, q, h + 1.0/3.0) * 255.0).round() as u8;
    let g = (hue_to_rgb(p, q, h) * 255.0).round() as u8;
    let b = (hue_to_rgb(p, q, h - 1.0/3.0) * 255.0).round() as u8;
    
    (r, g, b)
}

/// Calculate relative luminance for WCAG contrast
fn luminance(r: u8, g: u8, b: u8) -> f64 {
    let to_linear = |c: u8| -> f64 {
        let c = c as f64 / 255.0;
        if c <= 0.03928 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
    };
    
    0.2126 * to_linear(r) + 0.7152 * to_linear(g) + 0.0722 * to_linear(b)
}

/// Convert Color Format
pub async fn convert(
    color: &str,
    to_format: &str,
    _from_format: Option<&str>,
) -> Result<ConvertOutput, String> {
    let (r, g, b) = parse_hex(color)?;
    let (h, s, l) = rgb_to_hsl(r, g, b);
    
    let color_str = match to_format.to_lowercase().as_str() {
        "hex" => rgb_to_hex(r, g, b),
        "rgb" => format!("rgb({}, {}, {})", r, g, b),
        "hsl" => format!("hsl({:.0}, {:.0}%, {:.0}%)", h, s * 100.0, l * 100.0),
        _ => return Err(format!("Unsupported format: {}", to_format)),
    };
    
    Ok(ConvertOutput {
        color: color_str,
        components: ColorComponents {
            r: Some(r),
            g: Some(g),
            b: Some(b),
            h: Some(h),
            s: Some(s),
            l: Some(l),
            v: None,
            c: None,
            m: None,
            y: None,
            k: None,
        },
    })
}

/// Parse Color String
pub async fn parse(
    color: &str,
) -> Result<ParseOutput, String> {
    let (r, g, b) = match parse_hex(color) {
        Ok(rgb) => rgb,
        Err(_) => return Ok(ParseOutput {
            valid: false,
            format: String::new(),
            hex: String::new(),
            rgb: RgbColor { r: 0, g: 0, b: 0, a: None },
            hsl: HslColor { h: 0.0, s: 0.0, l: 0.0, a: None },
        }),
    };
    
    let (h, s, l) = rgb_to_hsl(r, g, b);
    
    Ok(ParseOutput {
        valid: true,
        format: "hex".to_string(),
        hex: rgb_to_hex(r, g, b),
        rgb: RgbColor { r, g, b, a: None },
        hsl: HslColor { h, s, l, a: None },
    })
}

/// Generate Color Palette
pub async fn generate_palette(
    base_color: &str,
    palette_type: &str,
    count: Option<i32>,
) -> Result<GeneratePaletteOutput, String> {
    let (r, g, b) = parse_hex(base_color)?;
    let (h, s, l) = rgb_to_hsl(r, g, b);
    let count = count.unwrap_or(5) as usize;
    
    let colors: Vec<String> = match palette_type.to_lowercase().as_str() {
        "complementary" => {
            let comp_h = (h + 180.0) % 360.0;
            vec![
                rgb_to_hex(r, g, b),
                {
                    let (r, g, b) = hsl_to_rgb(comp_h, s, l);
                    rgb_to_hex(r, g, b)
                },
            ]
        }
        "analogous" => {
            (0..count).map(|i| {
                let offset = (i as f64 - (count as f64 / 2.0)) * 30.0;
                let new_h = (h + offset + 360.0) % 360.0;
                let (r, g, b) = hsl_to_rgb(new_h, s, l);
                rgb_to_hex(r, g, b)
            }).collect()
        }
        "triadic" => {
            vec![
                rgb_to_hex(r, g, b),
                {
                    let (r, g, b) = hsl_to_rgb((h + 120.0) % 360.0, s, l);
                    rgb_to_hex(r, g, b)
                },
                {
                    let (r, g, b) = hsl_to_rgb((h + 240.0) % 360.0, s, l);
                    rgb_to_hex(r, g, b)
                },
            ]
        }
        "monochromatic" => {
            (0..count).map(|i| {
                let new_l = (l * (0.5 + (i as f64 / count as f64))).min(1.0);
                let (r, g, b) = hsl_to_rgb(h, s, new_l);
                rgb_to_hex(r, g, b)
            }).collect()
        }
        _ => return Err(format!("Unknown palette type: {}", palette_type)),
    };
    
    Ok(GeneratePaletteOutput {
        colors,
        palette_type: palette_type.to_string(),
    })
}

/// Lighten Color
pub async fn lighten(
    amount: f64,
    color: &str,
) -> Result<LightenOutput, String> {
    let (r, g, b) = parse_hex(color)?;
    let (h, s, l) = rgb_to_hsl(r, g, b);
    let new_l = (l + amount).min(1.0).max(0.0);
    let (r, g, b) = hsl_to_rgb(h, s, new_l);
    
    Ok(LightenOutput {
        color: rgb_to_hex(r, g, b),
    })
}

/// Darken Color
pub async fn darken(
    amount: f64,
    color: &str,
) -> Result<DarkenOutput, String> {
    let (r, g, b) = parse_hex(color)?;
    let (h, s, l) = rgb_to_hsl(r, g, b);
    let new_l = (l - amount).min(1.0).max(0.0);
    let (r, g, b) = hsl_to_rgb(h, s, new_l);
    
    Ok(DarkenOutput {
        color: rgb_to_hex(r, g, b),
    })
}

/// Saturate Color
pub async fn saturate(
    amount: f64,
    color: &str,
) -> Result<SaturateOutput, String> {
    let (r, g, b) = parse_hex(color)?;
    let (h, s, l) = rgb_to_hsl(r, g, b);
    let new_s = (s + amount).min(1.0).max(0.0);
    let (r, g, b) = hsl_to_rgb(h, new_s, l);
    
    Ok(SaturateOutput {
        color: rgb_to_hex(r, g, b),
    })
}

/// Desaturate Color
pub async fn desaturate(
    amount: f64,
    color: &str,
) -> Result<DesaturateOutput, String> {
    let (r, g, b) = parse_hex(color)?;
    let (h, s, l) = rgb_to_hsl(r, g, b);
    let new_s = (s - amount).min(1.0).max(0.0);
    let (r, g, b) = hsl_to_rgb(h, new_s, l);
    
    Ok(DesaturateOutput {
        color: rgb_to_hex(r, g, b),
    })
}

/// Mix Two Colors
pub async fn mix(
    color1: &str,
    color2: &str,
    weight: Option<f64>,
) -> Result<MixOutput, String> {
    let (r1, g1, b1) = parse_hex(color1)?;
    let (r2, g2, b2) = parse_hex(color2)?;
    let w = weight.unwrap_or(0.5);
    
    let r = ((r1 as f64 * w) + (r2 as f64 * (1.0 - w))).round() as u8;
    let g = ((g1 as f64 * w) + (g2 as f64 * (1.0 - w))).round() as u8;
    let b = ((b1 as f64 * w) + (b2 as f64 * (1.0 - w))).round() as u8;
    
    Ok(MixOutput {
        color: rgb_to_hex(r, g, b),
    })
}

/// Get Color Contrast Ratio
pub async fn contrast_ratio(
    color1: &str,
    color2: &str,
) -> Result<ContrastRatioOutput, String> {
    let (r1, g1, b1) = parse_hex(color1)?;
    let (r2, g2, b2) = parse_hex(color2)?;
    
    let l1 = luminance(r1, g1, b1);
    let l2 = luminance(r2, g2, b2);
    
    let ratio = if l1 > l2 {
        (l1 + 0.05) / (l2 + 0.05)
    } else {
        (l2 + 0.05) / (l1 + 0.05)
    };
    
    Ok(ContrastRatioOutput {
        ratio,
        wcag_aa: ratio >= 4.5,
        wcag_aaa: ratio >= 7.0,
    })
}

/// Get Accessible Text Color
pub async fn accessible_text_color(
    background_color: &str,
    prefer_white: Option<bool>,
) -> Result<AccessibleTextColorOutput, String> {
    let (r, g, b) = parse_hex(background_color)?;
    let bg_luminance = luminance(r, g, b);
    
    let white_contrast = (1.0 + 0.05) / (bg_luminance + 0.05);
    let black_contrast = (bg_luminance + 0.05) / (0.0 + 0.05);
    
    let prefer = prefer_white.unwrap_or(false);
    let (text_color, contrast) = if prefer && white_contrast >= 4.5 {
        ("#ffffff", white_contrast)
    } else if white_contrast > black_contrast {
        ("#ffffff", white_contrast)
    } else {
        ("#000000", black_contrast)
    };
    
    Ok(AccessibleTextColorOutput {
        text_color: text_color.to_string(),
        contrast_ratio: contrast,
    })
}

/// Invert Color
pub async fn invert(
    color: &str,
) -> Result<InvertOutput, String> {
    let (r, g, b) = parse_hex(color)?;
    
    Ok(InvertOutput {
        color: rgb_to_hex(255 - r, 255 - g, 255 - b),
    })
}

/// Generate Gradient Colors
pub async fn gradient(
    start_color: &str,
    end_color: &str,
    steps: i32,
    _color_space: Option<&str>,
) -> Result<GradientOutput, String> {
    if steps < 2 {
        return Err("Steps must be at least 2".to_string());
    }
    
    let (r1, g1, b1) = parse_hex(start_color)?;
    let (r2, g2, b2) = parse_hex(end_color)?;
    
    let colors: Vec<String> = (0..steps).map(|i| {
        let t = i as f64 / (steps - 1) as f64;
        let r = ((r1 as f64 * (1.0 - t)) + (r2 as f64 * t)).round() as u8;
        let g = ((g1 as f64 * (1.0 - t)) + (g2 as f64 * t)).round() as u8;
        let b = ((b1 as f64 * (1.0 - t)) + (b2 as f64 * t)).round() as u8;
        rgb_to_hex(r, g, b)
    }).collect();
    
    let css = format!(
        "linear-gradient(to right, {})",
        colors.join(", ")
    );
    
    Ok(GradientOutput { colors, css })
}

/// Get Color Luminance
pub async fn get_luminance(
    color: &str,
) -> Result<LuminanceOutput, String> {
    let (r, g, b) = parse_hex(color)?;
    let lum = luminance(r, g, b);
    
    // Using 0.179 as threshold (middle of perceptual range)
    Ok(LuminanceOutput {
        luminance: lum,
        is_dark: lum < 0.179,
        is_light: lum >= 0.179,
    })
}

/// Validate Color Format
pub async fn validate(
    color: &str,
    format: Option<&str>,
) -> Result<ValidateOutput, String> {
    // Try to detect the format
    let trimmed = color.trim();
    
    // Check hex format
    let is_hex = if trimmed.starts_with('#') {
        let hex = trimmed.trim_start_matches('#');
        (hex.len() == 3 || hex.len() == 6) && hex.chars().all(|c| c.is_ascii_hexdigit())
    } else {
        false
    };
    
    // Check rgb format
    let is_rgb = trimmed.to_lowercase().starts_with("rgb(") && trimmed.ends_with(')');
    
    // Check hsl format
    let is_hsl = trimmed.to_lowercase().starts_with("hsl(") && trimmed.ends_with(')');
    
    let detected_format = if is_hex {
        "hex"
    } else if is_rgb {
        "rgb"
    } else if is_hsl {
        "hsl"
    } else {
        "unknown"
    };
    
    let valid = match format {
        Some(f) => detected_format == f.to_lowercase(),
        None => detected_format != "unknown",
    };
    
    Ok(ValidateOutput {
        valid,
        detected_format: detected_format.to_string(),
    })
}

/// Generate Random Color
pub async fn random(
    format: Option<&str>,
    hue: Option<&str>,
    luminosity: Option<&str>,
) -> Result<RandomOutput, String> {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    // Simple pseudo-random based on time
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    
    // Simple LCG random number generator
    let rand = |seed: u64| -> u64 {
        seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407)
    };
    
    let s1 = rand(seed);
    let s2 = rand(s1);
    let s3 = rand(s2);
    
    // Generate base hue based on hue parameter
    let h: f64 = match hue {
        Some("red") => 0.0 + (s1 % 30) as f64,
        Some("orange") => 30.0 + (s1 % 30) as f64,
        Some("yellow") => 60.0 + (s1 % 30) as f64,
        Some("green") => 120.0 + (s1 % 60) as f64,
        Some("blue") => 210.0 + (s1 % 45) as f64,
        Some("purple") => 270.0 + (s1 % 30) as f64,
        Some("pink") => 300.0 + (s1 % 30) as f64,
        Some("monochrome") => 0.0, // Monochrome means no saturation
        _ => (s1 % 360) as f64,
    };
    
    // Generate saturation and lightness based on luminosity
    let (s, l) = match luminosity.unwrap_or("random") {
        "bright" => (0.7 + (s2 % 30) as f64 / 100.0, 0.5),
        "light" => (0.3 + (s2 % 40) as f64 / 100.0, 0.7 + (s3 % 20) as f64 / 100.0),
        "dark" => (0.5 + (s2 % 30) as f64 / 100.0, 0.2 + (s3 % 20) as f64 / 100.0),
        _ => ((s2 % 100) as f64 / 100.0, 0.3 + (s3 % 50) as f64 / 100.0),
    };
    
    // For monochrome, set saturation to 0
    let s = if hue == Some("monochrome") { 0.0 } else { s };
    
    let (r, g, b) = hsl_to_rgb(h, s, l);
    
    let color = match format.unwrap_or("hex").to_lowercase().as_str() {
        "rgb" => format!("rgb({}, {}, {})", r, g, b),
        "hsl" => format!("hsl({:.0}, {:.0}%, {:.0}%)", h, s * 100.0, l * 100.0),
        _ => rgb_to_hex(r, g, b),
    };
    
    Ok(RandomOutput { color })
}

#[cfg(test)]
mod tests;
