// Harana Actions - Image Module Tests

use super::*;
use base64::{Engine as _, engine::general_purpose::STANDARD};

// Create a simple 4x4 PNG image for testing
fn create_test_image() -> String {
    // Create a simple 4x4 RGBA image
    let img = image::RgbaImage::from_fn(4, 4, |x, y| {
        if (x + y) % 2 == 0 {
            image::Rgba([255, 0, 0, 255]) // Red
        } else {
            image::Rgba([0, 0, 255, 255]) // Blue
        }
    });
    
    let dynamic_img = DynamicImage::ImageRgba8(img);
    let mut buffer = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut buffer);
    dynamic_img.write_to(&mut cursor, ImageFormat::Png).unwrap();
    STANDARD.encode(&buffer)
}

// Create a larger image for some tests
fn create_test_image_sized(width: u32, height: u32) -> String {
    let img = image::RgbaImage::from_fn(width, height, |x, y| {
        image::Rgba([
            ((x * 255) / width.max(1)) as u8,
            ((y * 255) / height.max(1)) as u8,
            128,
            255,
        ])
    });
    
    let dynamic_img = DynamicImage::ImageRgba8(img);
    let mut buffer = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut buffer);
    dynamic_img.write_to(&mut cursor, ImageFormat::Png).unwrap();
    STANDARD.encode(&buffer)
}

// ============================================================================
// blur tests
// ============================================================================

#[tokio::test]
async fn test_blur() {
    let source = create_test_image();
    let result = blur(&source, Some(2.0)).await;
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(!output.image.is_empty());
}

#[tokio::test]
async fn test_blur_default_sigma() {
    let source = create_test_image();
    let result = blur(&source, None).await;
    assert!(result.is_ok());
}

// ============================================================================
// brightness tests
// ============================================================================

#[tokio::test]
async fn test_brightness_increase() {
    let source = create_test_image();
    let result = brightness(&source, 50.0).await;
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(!output.image.is_empty());
}

#[tokio::test]
async fn test_brightness_decrease() {
    let source = create_test_image();
    let result = brightness(&source, -50.0).await;
    assert!(result.is_ok());
}

// ============================================================================
// compress tests
// ============================================================================

#[tokio::test]
async fn test_compress() {
    let source = create_test_image_sized(100, 100);
    let result = compress(&source, None, Some(50), None).await;
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(!output.image.is_empty());
    assert!(output.size > 0);
}

#[tokio::test]
async fn test_compress_with_format() {
    let source = create_test_image_sized(100, 100);
    let result = compress(&source, None, Some(80), Some("png")).await;
    assert!(result.is_ok());
}

// ============================================================================
// contrast tests
// ============================================================================

#[tokio::test]
async fn test_contrast_increase() {
    let source = create_test_image();
    let result = contrast(&source, 50.0).await;
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(!output.image.is_empty());
}

#[tokio::test]
async fn test_contrast_decrease() {
    let source = create_test_image();
    let result = contrast(&source, -50.0).await;
    assert!(result.is_ok());
}

// ============================================================================
// convert_format tests
// ============================================================================

#[tokio::test]
async fn test_convert_format_to_jpeg() {
    let source = create_test_image_sized(10, 10);
    let result = convert_format(&source, "jpeg", None).await;
    assert!(result.is_ok());
    let output = result.unwrap();
    assert_eq!(output.format, "jpeg");
}

#[tokio::test]
async fn test_convert_format_to_bmp() {
    let source = create_test_image_sized(10, 10);
    let result = convert_format(&source, "bmp", None).await;
    assert!(result.is_ok());
    let output = result.unwrap();
    assert_eq!(output.format, "bmp");
}

#[tokio::test]
async fn test_convert_format_invalid() {
    let source = create_test_image();
    let result = convert_format(&source, "invalid_format", None).await;
    assert!(result.is_err());
}

// ============================================================================
// crop tests
// ============================================================================

#[tokio::test]
async fn test_crop() {
    let source = create_test_image_sized(100, 100);
    let result = crop(10, 10, &source, 50, 50).await;
    assert!(result.is_ok());
    let output = result.unwrap();
    assert_eq!(output.width, 50);
    assert_eq!(output.height, 50);
}

// ============================================================================
// flip tests
// ============================================================================

#[tokio::test]
async fn test_flip_horizontal() {
    let source = create_test_image();
    let result = flip(&source, "horizontal").await;
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(!output.image.is_empty());
}

#[tokio::test]
async fn test_flip_vertical() {
    let source = create_test_image();
    let result = flip(&source, "vertical").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_flip_invalid_direction() {
    let source = create_test_image();
    let result = flip(&source, "diagonal").await;
    assert!(result.is_err());
}

// ============================================================================
// get_metadata tests
// ============================================================================

#[tokio::test]
async fn test_get_metadata() {
    let source = create_test_image_sized(100, 50);
    let result = get_metadata(&source).await;
    assert!(result.is_ok());
    let output = result.unwrap();
    assert_eq!(output.width, 100);
    assert_eq!(output.height, 50);
    assert!(output.has_alpha);
    assert_eq!(output.color_space, "rgba");
}

// ============================================================================
// greyscale tests
// ============================================================================

#[tokio::test]
async fn test_greyscale() {
    let source = create_test_image();
    let result = greyscale(&source).await;
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(!output.image.is_empty());
}

// ============================================================================
// resize tests
// ============================================================================

#[tokio::test]
async fn test_resize_by_width() {
    let source = create_test_image_sized(100, 50);
    let result = resize(&source, None, Some(50), None, None, None).await;
    assert!(result.is_ok());
    let output = result.unwrap();
    assert_eq!(output.width, 50);
    assert_eq!(output.height, 25); // Maintains aspect ratio
}

#[tokio::test]
async fn test_resize_by_height() {
    let source = create_test_image_sized(100, 50);
    let result = resize(&source, None, None, Some(25), None, None).await;
    assert!(result.is_ok());
    let output = result.unwrap();
    assert_eq!(output.height, 25);
    assert_eq!(output.width, 50); // Maintains aspect ratio
}

#[tokio::test]
async fn test_resize_no_aspect_ratio() {
    let source = create_test_image_sized(100, 50);
    let result = resize(&source, None, Some(60), Some(60), None, Some(false)).await;
    assert!(result.is_ok());
    let output = result.unwrap();
    assert_eq!(output.width, 60);
    assert_eq!(output.height, 60);
}

// ============================================================================
// rotate tests
// ============================================================================

#[tokio::test]
async fn test_rotate_90() {
    let source = create_test_image_sized(100, 50);
    let result = rotate(90, &source, None).await;
    assert!(result.is_ok());
    let output = result.unwrap();
    // After 90 degree rotation, width and height swap
    assert_eq!(output.width, 50);
    assert_eq!(output.height, 100);
}

#[tokio::test]
async fn test_rotate_180() {
    let source = create_test_image_sized(100, 50);
    let result = rotate(180, &source, None).await;
    assert!(result.is_ok());
    let output = result.unwrap();
    assert_eq!(output.width, 100);
    assert_eq!(output.height, 50);
}

#[tokio::test]
async fn test_rotate_270() {
    let source = create_test_image_sized(100, 50);
    let result = rotate(270, &source, None).await;
    assert!(result.is_ok());
    let output = result.unwrap();
    assert_eq!(output.width, 50);
    assert_eq!(output.height, 100);
}

#[tokio::test]
async fn test_rotate_invalid_angle() {
    let source = create_test_image();
    let result = rotate(45, &source, None).await;
    assert!(result.is_err());
}

// ============================================================================
// sharpen tests
// ============================================================================

#[tokio::test]
async fn test_sharpen() {
    let source = create_test_image();
    let result = sharpen(&source, Some(1.5)).await;
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(!output.image.is_empty());
}

#[tokio::test]
async fn test_sharpen_default_sigma() {
    let source = create_test_image();
    let result = sharpen(&source, None).await;
    assert!(result.is_ok());
}

// ============================================================================
// thumbnail tests
// ============================================================================

#[tokio::test]
async fn test_thumbnail() {
    let source = create_test_image_sized(200, 100);
    let result = thumbnail(&source, None, Some(50), None).await;
    assert!(result.is_ok());
    let output = result.unwrap();
    // Thumbnail maintains aspect ratio, so dimensions should be <= 50
    assert!(output.width <= 50);
    assert!(output.height <= 50);
}

#[tokio::test]
async fn test_thumbnail_exact() {
    let source = create_test_image_sized(200, 100);
    let result = thumbnail(&source, Some("exact"), Some(50), None).await;
    assert!(result.is_ok());
    let output = result.unwrap();
    assert_eq!(output.width, 50);
    assert_eq!(output.height, 50);
}

// ============================================================================
// tint tests
// ============================================================================

#[tokio::test]
async fn test_tint_with_hash() {
    let source = create_test_image();
    let result = tint("#FF0000", &source).await;
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(!output.image.is_empty());
}

#[tokio::test]
async fn test_tint_without_hash() {
    let source = create_test_image();
    let result = tint("00FF00", &source).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_tint_invalid_color() {
    let source = create_test_image();
    let result = tint("invalid", &source).await;
    assert!(result.is_err());
}

// ============================================================================
// watermark tests
// ============================================================================

#[tokio::test]
async fn test_watermark() {
    let source = create_test_image_sized(100, 100);
    let watermark_img = create_test_image_sized(20, 20);
    let result = watermark(&source, &watermark_img, None, None).await;
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(!output.image.is_empty());
}

#[tokio::test]
async fn test_watermark_positions() {
    let source = create_test_image_sized(100, 100);
    let watermark_img = create_test_image_sized(20, 20);
    
    for pos in &["top-left", "top-right", "bottom-left", "bottom-right", "center"] {
        let result = watermark(&source, &watermark_img, Some(pos), Some(0.5)).await;
        assert!(result.is_ok(), "Failed for position: {}", pos);
    }
}

// ============================================================================
// error handling tests
// ============================================================================

#[tokio::test]
async fn test_invalid_base64() {
    let result = blur("not-valid-base64!!!", None).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_invalid_image_data() {
    let invalid_data = STANDARD.encode(b"not an image");
    let result = blur(&invalid_data, None).await;
    assert!(result.is_err());
}
