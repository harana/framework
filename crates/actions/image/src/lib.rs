// Harana Actions - Image Module
// This module provides image actions and functionality.

#![warn(missing_docs)]

pub mod output;

#[cfg(test)]
mod tests;

use output::*;
use image::{DynamicImage, ImageFormat, GenericImageView, imageops};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use std::io::Cursor;

/// Helper function to decode base64 image data
fn decode_image(source: &str) -> Result<DynamicImage, String> {
    let bytes = STANDARD.decode(source)
        .map_err(|e| format!("Failed to decode base64: {}", e))?;
    image::load_from_memory(&bytes)
        .map_err(|e| format!("Failed to load image: {}", e))
}

/// Helper function to encode image to base64
fn encode_image(img: &DynamicImage, format: ImageFormat) -> Result<String, String> {
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);
    img.write_to(&mut cursor, format)
        .map_err(|e| format!("Failed to encode image: {}", e))?;
    Ok(STANDARD.encode(&buffer))
}

/// Helper function to guess image format from base64 data
fn guess_format(source: &str) -> Result<ImageFormat, String> {
    let bytes = STANDARD.decode(source)
        .map_err(|e| format!("Failed to decode base64: {}", e))?;
    image::guess_format(&bytes)
        .map_err(|e| format!("Failed to guess image format: {}", e))
}

/// Blur Image
pub async fn blur(
    source: &str,
    sigma: Option<f64>,
) -> Result<BlurOutput, String> {
    let sigma = sigma.unwrap_or(1.0) as f32;
    let img = decode_image(source)?;
    let format = guess_format(source)?;
    let blurred = img.blur(sigma);
    let encoded = encode_image(&blurred, format)?;
    Ok(BlurOutput { image: encoded })
}

/// Adjust Brightness
pub async fn brightness(
    source: &str,
    brightness: f64,
) -> Result<BrightnessOutput, String> {
    let img = decode_image(source)?;
    let format = guess_format(source)?;
    // brightness is in range -100 to 100, image crate uses -255 to 255
    let adjusted = img.brighten((brightness * 2.55) as i32);
    let encoded = encode_image(&adjusted, format)?;
    Ok(BrightnessOutput { image: encoded })
}

/// Compress Image
pub async fn compress(
    source: &str,
    _progressive: Option<bool>,
    quality: Option<i32>,
    format: Option<&str>,
) -> Result<CompressOutput, String> {
    let _quality = quality.unwrap_or(80);
    let img = decode_image(source)?;
    let original_bytes = STANDARD.decode(source)
        .map_err(|e| format!("Failed to decode base64: {}", e))?;
    let original_size = original_bytes.len();
    
    let target_format = match format {
        Some("jpeg") | Some("jpg") => ImageFormat::Jpeg,
        Some("png") => ImageFormat::Png,
        Some("webp") => ImageFormat::WebP,
        Some("gif") => ImageFormat::Gif,
        _ => guess_format(source)?,
    };
    
    let encoded = encode_image(&img, target_format)?;
    let new_size = STANDARD.decode(&encoded)
        .map(|b| b.len())
        .unwrap_or(0);
    
    let compression_ratio = if original_size > 0 {
        1.0 - (new_size as f64 / original_size as f64)
    } else {
        0.0
    };
    
    Ok(CompressOutput {
        compression_ratio,
        size: new_size as i32,
        image: encoded,
    })
}

/// Adjust Contrast
pub async fn contrast(
    source: &str,
    contrast: f64,
) -> Result<ContrastOutput, String> {
    let img = decode_image(source)?;
    let format = guess_format(source)?;
    // contrast is a multiplier, image crate uses -100 to 100 range
    let adjusted = img.adjust_contrast(contrast as f32);
    let encoded = encode_image(&adjusted, format)?;
    Ok(ContrastOutput { image: encoded })
}

/// Convert Image Format
pub async fn convert_format(
    source: &str,
    format: &str,
    _quality: Option<i32>,
) -> Result<ConvertFormatOutput, String> {
    let img = decode_image(source)?;
    
    let target_format = match format.to_lowercase().as_str() {
        "jpeg" | "jpg" => ImageFormat::Jpeg,
        "png" => ImageFormat::Png,
        "gif" => ImageFormat::Gif,
        "bmp" => ImageFormat::Bmp,
        "webp" => ImageFormat::WebP,
        "tiff" | "tif" => ImageFormat::Tiff,
        "ico" => ImageFormat::Ico,
        _ => return Err(format!("Unsupported format: {}", format)),
    };
    
    let encoded = encode_image(&img, target_format)?;
    let size = STANDARD.decode(&encoded)
        .map(|b| b.len())
        .unwrap_or(0);
    
    Ok(ConvertFormatOutput {
        size: size as i32,
        image: encoded,
        format: format.to_string(),
    })
}

/// Crop Image
pub async fn crop(
    x: i32,
    y: i32,
    source: &str,
    height: i32,
    width: i32,
) -> Result<CropOutput, String> {
    let mut img = decode_image(source)?;
    let format = guess_format(source)?;
    
    let cropped = img.crop(x as u32, y as u32, width as u32, height as u32);
    let encoded = encode_image(&cropped, format)?;
    
    Ok(CropOutput {
        height,
        image: encoded,
        width,
    })
}

/// Flip Image
pub async fn flip(
    source: &str,
    direction: &str,
) -> Result<FlipOutput, String> {
    let img = decode_image(source)?;
    let format = guess_format(source)?;
    
    let flipped = match direction.to_lowercase().as_str() {
        "horizontal" | "h" => img.fliph(),
        "vertical" | "v" => img.flipv(),
        _ => return Err(format!("Invalid direction: {}. Use 'horizontal' or 'vertical'", direction)),
    };
    
    let encoded = encode_image(&flipped, format)?;
    Ok(FlipOutput { image: encoded })
}

/// Get Image Metadata
pub async fn get_metadata(
    source: &str,
) -> Result<GetMetadataOutput, String> {
    let bytes = STANDARD.decode(source)
        .map_err(|e| format!("Failed to decode base64: {}", e))?;
    let size = bytes.len();
    
    let img = image::load_from_memory(&bytes)
        .map_err(|e| format!("Failed to load image: {}", e))?;
    
    let format = image::guess_format(&bytes)
        .map(|f| format!("{:?}", f).to_lowercase())
        .unwrap_or_else(|_| "unknown".to_string());
    
    let (width, height) = img.dimensions();
    let has_alpha = img.color().has_alpha();
    
    // Determine color space
    let color_space = match img.color() {
        image::ColorType::L8 | image::ColorType::L16 => "grayscale",
        image::ColorType::La8 | image::ColorType::La16 => "grayscale-alpha",
        image::ColorType::Rgb8 | image::ColorType::Rgb16 => "rgb",
        image::ColorType::Rgba8 | image::ColorType::Rgba16 => "rgba",
        _ => "unknown",
    }.to_string();
    
    Ok(GetMetadataOutput {
        height: height as i32,
        format,
        orientation: 1, // Default orientation (no EXIF support in basic image crate)
        width: width as i32,
        size: size as i32,
        has_alpha,
        color_space,
    })
}

/// Convert To Greyscale
pub async fn greyscale(
    source: &str,
) -> Result<GreyscaleOutput, String> {
    let img = decode_image(source)?;
    let format = guess_format(source)?;
    let grey = img.grayscale();
    let encoded = encode_image(&grey, format)?;
    Ok(GreyscaleOutput { image: encoded })
}

/// Resize Image
pub async fn resize(
    source: &str,
    format: Option<&str>,
    width: Option<i32>,
    height: Option<i32>,
    fit: Option<&str>,
    maintain_aspect_ratio: Option<bool>,
) -> Result<ResizeOutput, String> {
    let img = decode_image(source)?;
    let original_format = guess_format(source)?;
    let (orig_width, orig_height) = img.dimensions();
    
    let maintain_ratio = maintain_aspect_ratio.unwrap_or(true);
    let fit_mode = fit.unwrap_or("contain");
    
    // Calculate target dimensions
    let (target_width, target_height) = match (width, height, maintain_ratio) {
        (Some(w), Some(h), false) => (w as u32, h as u32),
        (Some(w), Some(h), true) => {
            // Calculate to maintain aspect ratio
            let ratio_w = w as f64 / orig_width as f64;
            let ratio_h = h as f64 / orig_height as f64;
            let ratio = match fit_mode {
                "cover" => ratio_w.max(ratio_h),
                _ => ratio_w.min(ratio_h), // "contain" is default
            };
            ((orig_width as f64 * ratio) as u32, (orig_height as f64 * ratio) as u32)
        }
        (Some(w), None, _) => {
            let ratio = w as f64 / orig_width as f64;
            (w as u32, (orig_height as f64 * ratio) as u32)
        }
        (None, Some(h), _) => {
            let ratio = h as f64 / orig_height as f64;
            ((orig_width as f64 * ratio) as u32, h as u32)
        }
        (None, None, _) => (orig_width, orig_height),
    };
    
    let resized = img.resize(target_width, target_height, imageops::FilterType::Lanczos3);
    
    let target_format = match format {
        Some("jpeg") | Some("jpg") => ImageFormat::Jpeg,
        Some("png") => ImageFormat::Png,
        Some("webp") => ImageFormat::WebP,
        Some("gif") => ImageFormat::Gif,
        _ => original_format,
    };
    
    let encoded = encode_image(&resized, target_format)?;
    let size = STANDARD.decode(&encoded)
        .map(|b| b.len())
        .unwrap_or(0);
    
    Ok(ResizeOutput {
        image: encoded,
        height: target_height as i32,
        size: size as i32,
        width: target_width as i32,
    })
}

/// Rotate Image
pub async fn rotate(
    angle: i32,
    source: &str,
    _background: Option<&str>,
) -> Result<RotateOutput, String> {
    let img = decode_image(source)?;
    let format = guess_format(source)?;
    
    let rotated = match angle % 360 {
        90 | -270 => img.rotate90(),
        180 | -180 => img.rotate180(),
        270 | -90 => img.rotate270(),
        0 => img,
        _ => return Err(format!("Rotation angle must be 0, 90, 180, or 270 degrees. Got: {}", angle)),
    };
    
    let (width, height) = rotated.dimensions();
    let encoded = encode_image(&rotated, format)?;
    
    Ok(RotateOutput {
        image: encoded,
        width: width as i32,
        height: height as i32,
    })
}

/// Apply Sharpen
pub async fn sharpen(
    source: &str,
    sigma: Option<f64>,
) -> Result<SharpenOutput, String> {
    let sigma = sigma.unwrap_or(1.0) as f32;
    let img = decode_image(source)?;
    let format = guess_format(source)?;
    let sharpened = img.unsharpen(sigma, 1);
    let encoded = encode_image(&sharpened, format)?;
    Ok(SharpenOutput { image: encoded })
}

/// Generate Thumbnail
pub async fn thumbnail(
    source: &str,
    fit: Option<&str>,
    size: Option<i32>,
    format: Option<&str>,
) -> Result<ThumbnailOutput, String> {
    let size = size.unwrap_or(150) as u32;
    let img = decode_image(source)?;
    let original_format = guess_format(source)?;
    
    let thumb = match fit {
        Some("exact") => img.thumbnail_exact(size, size),
        _ => img.thumbnail(size, size), // Default maintains aspect ratio
    };
    
    let (width, height) = thumb.dimensions();
    
    let target_format = match format {
        Some("jpeg") | Some("jpg") => ImageFormat::Jpeg,
        Some("png") => ImageFormat::Png,
        Some("webp") => ImageFormat::WebP,
        Some("gif") => ImageFormat::Gif,
        _ => original_format,
    };
    
    let encoded = encode_image(&thumb, target_format)?;
    
    Ok(ThumbnailOutput {
        image: encoded,
        width: width as i32,
        height: height as i32,
    })
}

/// Apply Tint
pub async fn tint(
    color: &str,
    source: &str,
) -> Result<TintOutput, String> {
    let img = decode_image(source)?;
    let format = guess_format(source)?;
    
    // Parse color (supports #RRGGBB or RRGGBB)
    let color_str = color.trim_start_matches('#');
    if color_str.len() != 6 {
        return Err(format!("Invalid color format: {}. Use #RRGGBB or RRGGBB", color));
    }
    
    let r = u8::from_str_radix(&color_str[0..2], 16)
        .map_err(|_| format!("Invalid red component in color: {}", color))?;
    let g = u8::from_str_radix(&color_str[2..4], 16)
        .map_err(|_| format!("Invalid green component in color: {}", color))?;
    let b = u8::from_str_radix(&color_str[4..6], 16)
        .map_err(|_| format!("Invalid blue component in color: {}", color))?;
    
    // Apply tint by blending with the color
    let mut rgba_img = img.to_rgba8();
    for pixel in rgba_img.pixels_mut() {
        // Blend with tint color (50% blend)
        pixel[0] = ((pixel[0] as u16 + r as u16) / 2) as u8;
        pixel[1] = ((pixel[1] as u16 + g as u16) / 2) as u8;
        pixel[2] = ((pixel[2] as u16 + b as u16) / 2) as u8;
    }
    
    let tinted = DynamicImage::ImageRgba8(rgba_img);
    let encoded = encode_image(&tinted, format)?;
    
    Ok(TintOutput { image: encoded })
}

/// Add Watermark
pub async fn watermark(
    source: &str,
    watermark: &str,
    position: Option<&str>,
    opacity: Option<f64>,
) -> Result<WatermarkOutput, String> {
    let img = decode_image(source)?;
    let format = guess_format(source)?;
    let watermark_img = decode_image(watermark)?;
    let opacity = opacity.unwrap_or(0.5);
    
    let (img_width, img_height) = img.dimensions();
    let (wm_width, wm_height) = watermark_img.dimensions();
    
    // Calculate position
    let (x, y) = match position.unwrap_or("bottom-right") {
        "top-left" => (0u32, 0u32),
        "top-right" => (img_width.saturating_sub(wm_width), 0),
        "bottom-left" => (0, img_height.saturating_sub(wm_height)),
        "bottom-right" => (img_width.saturating_sub(wm_width), img_height.saturating_sub(wm_height)),
        "center" => ((img_width.saturating_sub(wm_width)) / 2, (img_height.saturating_sub(wm_height)) / 2),
        _ => (img_width.saturating_sub(wm_width), img_height.saturating_sub(wm_height)),
    };
    
    // Overlay watermark with opacity
    let mut result = img.to_rgba8();
    let watermark_rgba = watermark_img.to_rgba8();
    
    for (wx, wy, wm_pixel) in watermark_rgba.enumerate_pixels() {
        let px = x + wx;
        let py = y + wy;
        if px < img_width && py < img_height {
            let base_pixel = result.get_pixel(px, py);
            let alpha = (wm_pixel[3] as f64 / 255.0) * opacity;
            let inv_alpha = 1.0 - alpha;
            
            let blended = image::Rgba([
                (base_pixel[0] as f64 * inv_alpha + wm_pixel[0] as f64 * alpha) as u8,
                (base_pixel[1] as f64 * inv_alpha + wm_pixel[1] as f64 * alpha) as u8,
                (base_pixel[2] as f64 * inv_alpha + wm_pixel[2] as f64 * alpha) as u8,
                255,
            ]);
            result.put_pixel(px, py, blended);
        }
    }
    
    let watermarked = DynamicImage::ImageRgba8(result);
    let encoded = encode_image(&watermarked, format)?;
    
    Ok(WatermarkOutput { image: encoded })
}
