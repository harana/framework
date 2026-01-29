#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test]
    async fn test_parse_hex() {
        let result = parse("#ff5500").await.unwrap();
        assert!(result.valid);
        assert_eq!(result.rgb.r, 255);
        assert_eq!(result.rgb.g, 85);
        assert_eq!(result.rgb.b, 0);
    }

    #[tokio::test]
    async fn test_parse_short_hex() {
        let result = parse("#f50").await.unwrap();
        assert!(result.valid);
        assert_eq!(result.rgb.r, 255);
        assert_eq!(result.rgb.g, 85);
        assert_eq!(result.rgb.b, 0);
    }

    #[tokio::test]
    async fn test_parse_invalid() {
        let result = parse("not-a-color").await.unwrap();
        assert!(!result.valid);
    }

    #[tokio::test]
    async fn test_convert_to_rgb() {
        let result = convert("#ff0000", "rgb", None).await.unwrap();
        assert_eq!(result.color, "rgb(255, 0, 0)");
    }

    #[tokio::test]
    async fn test_convert_to_hsl() {
        let result = convert("#ff0000", "hsl", None).await.unwrap();
        assert!(result.color.starts_with("hsl(0,"));
    }

    #[tokio::test]
    async fn test_lighten() {
        let result = lighten(0.2, "#808080").await.unwrap();
        // Gray lightened should be lighter
        let parsed = parse(&result.color).await.unwrap();
        assert!(parsed.hsl.l > 0.5);
    }

    #[tokio::test]
    async fn test_darken() {
        let result = darken(0.2, "#808080").await.unwrap();
        // Gray darkened should be darker
        let parsed = parse(&result.color).await.unwrap();
        assert!(parsed.hsl.l < 0.5);
    }

    #[tokio::test]
    async fn test_invert() {
        let result = invert("#000000").await.unwrap();
        assert_eq!(result.color, "#ffffff");
        
        let result = invert("#ffffff").await.unwrap();
        assert_eq!(result.color, "#000000");
    }

    #[tokio::test]
    async fn test_mix() {
        let result = mix("#ff0000", "#0000ff", Some(0.5)).await.unwrap();
        let parsed = parse(&result.color).await.unwrap();
        // Mix should give purple-ish
        assert!(parsed.rgb.r > 0);
        assert!(parsed.rgb.b > 0);
    }

    #[tokio::test]
    async fn test_contrast_ratio() {
        let result = contrast_ratio("#ffffff", "#000000").await.unwrap();
        // White and black should have max contrast
        assert!(result.ratio > 20.0);
        assert!(result.wcag_aa);
        assert!(result.wcag_aaa);
    }

    #[tokio::test]
    async fn test_accessible_text_color_light_bg() {
        let result = accessible_text_color("#ffffff", None).await.unwrap();
        // On white background, black text should be chosen
        assert_eq!(result.text_color, "#000000");
    }

    #[tokio::test]
    async fn test_accessible_text_color_dark_bg() {
        let result = accessible_text_color("#000000", None).await.unwrap();
        // On black background, white text should be chosen
        assert_eq!(result.text_color, "#ffffff");
    }

    #[tokio::test]
    async fn test_gradient() {
        let result = gradient("#ff0000", "#0000ff", 5, None).await.unwrap();
        assert_eq!(result.colors.len(), 5);
        assert_eq!(result.colors[0], "#ff0000");
        assert_eq!(result.colors[4], "#0000ff");
        assert!(result.css.contains("linear-gradient"));
    }

    #[tokio::test]
    async fn test_generate_palette_complementary() {
        let result = generate_palette("#ff0000", "complementary", None).await.unwrap();
        assert_eq!(result.colors.len(), 2);
        assert_eq!(result.palette_type, "complementary");
    }

    #[tokio::test]
    async fn test_generate_palette_triadic() {
        let result = generate_palette("#ff0000", "triadic", None).await.unwrap();
        assert_eq!(result.colors.len(), 3);
    }

    #[tokio::test]
    async fn test_saturate() {
        let result = saturate(0.2, "#808080").await.unwrap();
        // Saturating gray doesn't change much since it has no hue
        assert!(!result.color.is_empty());
    }

    #[tokio::test]
    async fn test_desaturate() {
        let result = desaturate(0.5, "#ff0000").await.unwrap();
        let parsed = parse(&result.color).await.unwrap();
        // Desaturated red should have lower saturation
        assert!(parsed.hsl.s < 1.0);
    }

    #[tokio::test]
    async fn test_luminance_dark() {
        let result = get_luminance("#000000").await.unwrap();
        assert!(result.is_dark);
        assert!(!result.is_light);
        assert!(result.luminance < 0.01);
    }

    #[tokio::test]
    async fn test_luminance_light() {
        let result = get_luminance("#ffffff").await.unwrap();
        assert!(!result.is_dark);
        assert!(result.is_light);
        assert!(result.luminance > 0.9);
    }

    #[tokio::test]
    async fn test_validate_hex() {
        let result = validate("#ff0000", None).await.unwrap();
        assert!(result.valid);
        assert_eq!(result.detected_format, "hex");
    }

    #[tokio::test]
    async fn test_validate_hex_short() {
        let result = validate("#f00", None).await.unwrap();
        assert!(result.valid);
        assert_eq!(result.detected_format, "hex");
    }

    #[tokio::test]
    async fn test_validate_rgb() {
        let result = validate("rgb(255, 0, 0)", None).await.unwrap();
        assert!(result.valid);
        assert_eq!(result.detected_format, "rgb");
    }

    #[tokio::test]
    async fn test_validate_hsl() {
        let result = validate("hsl(0, 100%, 50%)", None).await.unwrap();
        assert!(result.valid);
        assert_eq!(result.detected_format, "hsl");
    }

    #[tokio::test]
    async fn test_validate_invalid() {
        let result = validate("not-a-color", None).await.unwrap();
        assert!(!result.valid);
        assert_eq!(result.detected_format, "unknown");
    }

    #[tokio::test]
    async fn test_validate_with_format() {
        let result = validate("#ff0000", Some("hex")).await.unwrap();
        assert!(result.valid);
        
        let result = validate("#ff0000", Some("rgb")).await.unwrap();
        assert!(!result.valid);
    }

    #[tokio::test]
    async fn test_random_hex() {
        let result = random(Some("hex"), None, None).await.unwrap();
        assert!(result.color.starts_with('#'));
        assert_eq!(result.color.len(), 7);
    }

    #[tokio::test]
    async fn test_random_rgb() {
        let result = random(Some("rgb"), None, None).await.unwrap();
        assert!(result.color.starts_with("rgb("));
    }

    #[tokio::test]
    async fn test_random_hsl() {
        let result = random(Some("hsl"), None, None).await.unwrap();
        assert!(result.color.starts_with("hsl("));
    }

    #[tokio::test]
    async fn test_random_with_hue() {
        let result = random(None, Some("red"), None).await.unwrap();
        assert!(result.color.starts_with('#'));
    }

    #[tokio::test]
    async fn test_random_with_luminosity() {
        let result = random(None, None, Some("bright")).await.unwrap();
        assert!(result.color.starts_with('#'));
        
        let result = random(None, None, Some("dark")).await.unwrap();
        assert!(result.color.starts_with('#'));
        
        let result = random(None, None, Some("light")).await.unwrap();
        assert!(result.color.starts_with('#'));
    }
}
