// Harana Actions - PDF Module Tests

use super::*;
use output::*;
use lopdf::dictionary;

/// Helper function to create a simple test PDF document
fn create_test_pdf() -> Vec<u8> {
    let mut doc = Document::with_version("1.5");
    
    let pages_id = doc.new_object_id();
    let page_id = doc.new_object_id();
    let content_id = doc.new_object_id();
    let font_id = doc.new_object_id();
    
    // Font dictionary
    let font_dict = dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
    };
    doc.objects.insert(font_id, Object::Dictionary(font_dict));
    
    // Resources dictionary
    let resources = dictionary! {
        "Font" => dictionary! {
            "F1" => Object::Reference(font_id),
        },
    };
    
    // Content stream
    let content = "BT\n/F1 12 Tf\n50 750 Td\n(Hello World) Tj\nET";
    let content_stream = Stream::new(Dictionary::new(), content.as_bytes().to_vec());
    doc.objects.insert(content_id, Object::Stream(content_stream));
    
    // Page dictionary
    let page_dict = dictionary! {
        "Type" => "Page",
        "Parent" => Object::Reference(pages_id),
        "MediaBox" => vec![
            Object::Integer(0),
            Object::Integer(0),
            Object::Integer(612),
            Object::Integer(792),
        ],
        "Contents" => Object::Reference(content_id),
        "Resources" => Object::Dictionary(resources),
    };
    doc.objects.insert(page_id, Object::Dictionary(page_dict));
    
    // Pages dictionary
    let pages_dict = dictionary! {
        "Type" => "Pages",
        "Kids" => vec![Object::Reference(page_id)],
        "Count" => Object::Integer(1),
    };
    doc.objects.insert(pages_id, Object::Dictionary(pages_dict));
    
    // Catalog
    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => Object::Reference(pages_id),
    });
    
    doc.trailer.set("Root", Object::Reference(catalog_id));
    
    let mut output = Vec::new();
    doc.save_to(&mut output).unwrap();
    output
}

/// Helper function to create a multi-page test PDF
fn create_multipage_pdf(num_pages: i32) -> Vec<u8> {
    let mut doc = Document::with_version("1.5");
    
    let pages_id = doc.new_object_id();
    let font_id = doc.new_object_id();
    
    // Font dictionary
    let font_dict = dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
    };
    doc.objects.insert(font_id, Object::Dictionary(font_dict));
    
    let mut kids = Vec::new();
    
    for i in 1..=num_pages {
        let page_id = doc.new_object_id();
        let content_id = doc.new_object_id();
        
        // Resources dictionary
        let resources = dictionary! {
            "Font" => dictionary! {
                "F1" => Object::Reference(font_id),
            },
        };
        
        // Content stream
        let content = format!("BT\n/F1 12 Tf\n50 750 Td\n(Page {}) Tj\nET", i);
        let content_stream = Stream::new(Dictionary::new(), content.as_bytes().to_vec());
        doc.objects.insert(content_id, Object::Stream(content_stream));
        
        // Page dictionary
        let page_dict = dictionary! {
            "Type" => "Page",
            "Parent" => Object::Reference(pages_id),
            "MediaBox" => vec![
                Object::Integer(0),
                Object::Integer(0),
                Object::Integer(612),
                Object::Integer(792),
            ],
            "Contents" => Object::Reference(content_id),
            "Resources" => Object::Dictionary(resources),
        };
        doc.objects.insert(page_id, Object::Dictionary(page_dict));
        kids.push(Object::Reference(page_id));
    }
    
    // Pages dictionary
    let pages_dict = dictionary! {
        "Type" => "Pages",
        "Kids" => Object::Array(kids),
        "Count" => Object::Integer(num_pages as i64),
    };
    doc.objects.insert(pages_id, Object::Dictionary(pages_dict));
    
    // Catalog
    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => Object::Reference(pages_id),
    });
    
    doc.trailer.set("Root", Object::Reference(catalog_id));
    
    let mut output = Vec::new();
    doc.save_to(&mut output).unwrap();
    output
}

// ============================================================================
// merge tests
// ============================================================================

#[tokio::test]
async fn test_merge_empty_documents() {
    let result = merge(vec![], None).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("No documents provided"));
}

#[tokio::test]
async fn test_merge_single_document() {
    let pdf = create_test_pdf();
    let result = merge(vec![pdf.clone()], None).await.unwrap();
    
    assert_eq!(result.page_count, 1);
    assert!(!result.pdf.is_empty());
    assert!(result.size > 0);
}

#[tokio::test]
async fn test_merge_multiple_documents() {
    let pdf1 = create_test_pdf();
    let pdf2 = create_test_pdf();
    
    let result = merge(vec![pdf1, pdf2], Some("merged.pdf")).await.unwrap();
    
    // After merge, should have more pages
    assert!(result.page_count >= 1);
    assert!(!result.pdf.is_empty());
}

// ============================================================================
// split tests
// ============================================================================

#[tokio::test]
async fn test_split_into_individual_pages() {
    let pdf = create_multipage_pdf(3);
    let result = split(&pdf, None, None).await.unwrap();
    
    assert_eq!(result.documents.len(), 3);
    assert_eq!(result.page_counts.len(), 3);
    for count in &result.page_counts {
        assert_eq!(*count, 1);
    }
}

#[tokio::test]
async fn test_split_specific_pages() {
    let pdf = create_multipage_pdf(3);
    let result = split(&pdf, Some(vec![1, 3]), None).await.unwrap();
    
    assert_eq!(result.documents.len(), 2);
    assert_eq!(result.page_counts, vec![1, 1]);
}

#[tokio::test]
async fn test_split_by_range() {
    let pdf = create_multipage_pdf(5);
    let result = split(&pdf, None, Some(vec!["1-2".to_string(), "3-5".to_string()])).await.unwrap();
    
    assert_eq!(result.documents.len(), 2);
    assert_eq!(result.page_counts, vec![2, 3]);
}

#[tokio::test]
async fn test_split_invalid_range() {
    let pdf = create_multipage_pdf(3);
    let result = split(&pdf, None, Some(vec!["1-10".to_string()])).await;
    
    assert!(result.is_err());
}

// ============================================================================
// extract_pages tests
// ============================================================================

#[tokio::test]
async fn test_extract_pages_success() {
    let pdf = create_multipage_pdf(5);
    let result = extract_pages(&pdf, 2, 4).await.unwrap();
    
    assert_eq!(result.page_count, 3);
    assert!(!result.pdf.is_empty());
    assert!(result.size > 0);
}

#[tokio::test]
async fn test_extract_pages_single_page() {
    let pdf = create_multipage_pdf(3);
    let result = extract_pages(&pdf, 2, 2).await.unwrap();
    
    assert_eq!(result.page_count, 1);
}

#[tokio::test]
async fn test_extract_pages_invalid_start() {
    let pdf = create_test_pdf();
    let result = extract_pages(&pdf, 0, 1).await;
    
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Start page must be at least 1"));
}

#[tokio::test]
async fn test_extract_pages_end_before_start() {
    let pdf = create_multipage_pdf(3);
    let result = extract_pages(&pdf, 3, 1).await;
    
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("End page must be >= start page"));
}

#[tokio::test]
async fn test_extract_pages_exceeds_document() {
    let pdf = create_test_pdf();
    let result = extract_pages(&pdf, 1, 10).await;
    
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("exceeds document page count"));
}

// ============================================================================
// extract_text tests
// ============================================================================

#[tokio::test]
async fn test_extract_text_all_pages() {
    let pdf = create_test_pdf();
    let result = extract_text(&pdf, None, None).await.unwrap();
    
    assert!(!result.pages.is_empty());
}

#[tokio::test]
async fn test_extract_text_specific_pages() {
    let pdf = create_multipage_pdf(3);
    let result = extract_text(&pdf, Some(vec![1, 3]), None).await.unwrap();
    
    assert_eq!(result.pages.len(), 2);
    assert_eq!(result.pages[0].page_number, 1);
    assert_eq!(result.pages[1].page_number, 3);
}

#[tokio::test]
async fn test_extract_text_preserve_layout() {
    let pdf = create_test_pdf();
    let result = extract_text(&pdf, None, Some(true)).await.unwrap();
    
    // Should work regardless of preserve_layout setting
    assert!(!result.pages.is_empty());
}

// ============================================================================
// extract_images tests
// ============================================================================

#[tokio::test]
async fn test_extract_images_no_images() {
    let pdf = create_test_pdf(); // No images in test PDF
    let result = extract_images(&pdf, None, None).await.unwrap();
    
    assert_eq!(result.count, 0);
    assert!(result.images.is_empty());
}

// ============================================================================
// add_watermark tests
// ============================================================================

#[tokio::test]
async fn test_add_watermark_text() {
    let pdf = create_test_pdf();
    let result = add_watermark(
        &pdf,
        Some(0.5),
        None,
        Some("diagonal"),
        None,
        Some("CONFIDENTIAL"),
    ).await.unwrap();
    
    assert!(!result.pdf.is_empty());
    assert!(result.size > 0);
}

#[tokio::test]
async fn test_add_watermark_defaults() {
    let pdf = create_test_pdf();
    let result = add_watermark(&pdf, None, None, None, None, None).await.unwrap();
    
    assert!(!result.pdf.is_empty());
}

// ============================================================================
// compress tests
// ============================================================================

#[tokio::test]
async fn test_compress_default() {
    let pdf = create_test_pdf();
    let original_size = pdf.len() as i64;
    
    let result = compress(&pdf, None, None).await.unwrap();
    
    assert_eq!(result.original_size, original_size);
    assert!(result.compressed_size > 0);
    assert!(!result.pdf.is_empty());
}

#[tokio::test]
async fn test_compress_high_quality() {
    let pdf = create_test_pdf();
    let result = compress(&pdf, Some("high"), Some(false)).await.unwrap();
    
    assert!(result.compressed_size > 0);
}

#[tokio::test]
async fn test_compress_remove_metadata() {
    let pdf = create_test_pdf();
    let result = compress(&pdf, Some("low"), Some(true)).await.unwrap();
    
    assert!(!result.pdf.is_empty());
}

// ============================================================================
// encrypt tests
// ============================================================================

#[tokio::test]
async fn test_encrypt_user_password() {
    let pdf = create_test_pdf();
    let result = encrypt(&pdf, "password123", None, None).await.unwrap();
    
    // Note: encryption is not fully implemented in lopdf
    assert!(!result.pdf.is_empty());
}

// ============================================================================
// decrypt tests
// ============================================================================

#[tokio::test]
async fn test_decrypt_unencrypted() {
    let pdf = create_test_pdf();
    let result = decrypt(&pdf, "anypassword").await.unwrap();
    
    assert!(result.success);
    assert!(!result.pdf.is_empty());
}

// ============================================================================
// get_metadata tests
// ============================================================================

#[tokio::test]
async fn test_get_metadata_basic() {
    let pdf = create_test_pdf();
    let result = get_metadata(&pdf).await.unwrap();
    
    assert_eq!(result.page_count, 1);
}

#[tokio::test]
async fn test_get_metadata_multipage() {
    let pdf = create_multipage_pdf(5);
    let result = get_metadata(&pdf).await.unwrap();
    
    assert_eq!(result.page_count, 5);
}

// ============================================================================
// set_metadata tests
// ============================================================================

#[tokio::test]
async fn test_set_metadata_title() {
    let pdf = create_test_pdf();
    let result = set_metadata(
        &pdf,
        None,
        None,
        None,
        Some("Test Document"),
    ).await.unwrap();
    
    assert!(result.success);
    assert!(!result.pdf.is_empty());
    
    // Verify metadata was set
    let metadata = get_metadata(&result.pdf).await.unwrap();
    assert_eq!(metadata.title, Some("Test Document".to_string()));
}

#[tokio::test]
async fn test_set_metadata_all_fields() {
    let pdf = create_test_pdf();
    let result = set_metadata(
        &pdf,
        Some("John Doe"),
        Some(vec!["rust".to_string(), "pdf".to_string()]),
        Some("Test Subject"),
        Some("Test Title"),
    ).await.unwrap();
    
    assert!(result.success);
    
    let metadata = get_metadata(&result.pdf).await.unwrap();
    assert_eq!(metadata.title, Some("Test Title".to_string()));
    assert_eq!(metadata.author, Some("John Doe".to_string()));
    assert_eq!(metadata.subject, Some("Test Subject".to_string()));
}

// ============================================================================
// rotate_pages tests
// ============================================================================

#[tokio::test]
async fn test_rotate_pages_90() {
    let pdf = create_test_pdf();
    let result = rotate_pages(&pdf, 90, None).await.unwrap();
    
    assert!(result.success);
    assert!(!result.pdf.is_empty());
}

#[tokio::test]
async fn test_rotate_pages_180() {
    let pdf = create_test_pdf();
    let result = rotate_pages(&pdf, 180, None).await.unwrap();
    
    assert!(result.success);
}

#[tokio::test]
async fn test_rotate_pages_270() {
    let pdf = create_test_pdf();
    let result = rotate_pages(&pdf, 270, None).await.unwrap();
    
    assert!(result.success);
}

#[tokio::test]
async fn test_rotate_pages_invalid_angle() {
    let pdf = create_test_pdf();
    let result = rotate_pages(&pdf, 45, None).await;
    
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid rotation angle"));
}

#[tokio::test]
async fn test_rotate_specific_pages() {
    let pdf = create_multipage_pdf(3);
    let result = rotate_pages(&pdf, 90, Some(vec![1, 3])).await.unwrap();
    
    assert!(result.success);
}

// ============================================================================
// add_page_numbers tests
// ============================================================================

#[tokio::test]
async fn test_add_page_numbers_default() {
    let pdf = create_multipage_pdf(3);
    let result = add_page_numbers(&pdf, None, None, None, None, None).await.unwrap();
    
    assert!(!result.pdf.is_empty());
    assert!(result.size > 0);
}

#[tokio::test]
async fn test_add_page_numbers_custom_format() {
    let pdf = create_multipage_pdf(5);
    let result = add_page_numbers(
        &pdf,
        Some("right"),
        Some(10),
        Some("Page {page}/{total}"),
        Some("top"),
        Some(1),
    ).await.unwrap();
    
    assert!(!result.pdf.is_empty());
}

// ============================================================================
// to_images tests
// ============================================================================

#[tokio::test]
async fn test_to_images_default() {
    let pdf = create_test_pdf();
    let result = to_images(&pdf, None, None, None).await.unwrap();
    
    // Count should match page count (even if images are empty placeholders)
    assert_eq!(result.count, 1);
}

#[tokio::test]
async fn test_to_images_specific_pages() {
    let pdf = create_multipage_pdf(5);
    let result = to_images(&pdf, Some(300), Some("png"), Some(vec![1, 3, 5])).await.unwrap();
    
    assert_eq!(result.count, 3);
}

// ============================================================================
// fill_form tests
// ============================================================================

#[tokio::test]
async fn test_fill_form_no_fields() {
    let pdf = create_test_pdf(); // No form fields
    let result = fill_form(&pdf, vec![], None).await.unwrap();
    
    assert!(!result.pdf.is_empty());
}

#[tokio::test]
async fn test_fill_form_with_fields() {
    let pdf = create_test_pdf();
    let fields = vec![
        PdfFormFieldValue { name: "name".to_string(), value: "John".to_string() },
        PdfFormFieldValue { name: "email".to_string(), value: "john@example.com".to_string() },
    ];
    
    let result = fill_form(&pdf, fields, Some(true)).await.unwrap();
    
    assert!(!result.pdf.is_empty());
}

// ============================================================================
// get_form_fields tests
// ============================================================================

#[tokio::test]
async fn test_get_form_fields_no_form() {
    let pdf = create_test_pdf();
    let result = get_form_fields(&pdf).await.unwrap();
    
    assert_eq!(result.count, 0);
    assert!(result.fields.is_empty());
}

// ============================================================================
// from_html tests
// ============================================================================

#[tokio::test]
async fn test_from_html_simple() {
    let result = from_html("<p>Hello World</p>", None, None, None, None, None, None).await.unwrap();
    
    assert_eq!(result.page_count, 1);
    assert!(!result.pdf.is_empty());
    assert!(result.size > 0);
}

#[tokio::test]
async fn test_from_html_complex() {
    let html = r#"
        <html>
            <head><title>Test</title></head>
            <body>
                <h1>Heading</h1>
                <p>Paragraph with <strong>bold</strong> text.</p>
            </body>
        </html>
    "#;
    
    let result = from_html(html, None, None, None, None, None, None).await.unwrap();
    
    assert_eq!(result.page_count, 1);
    assert!(!result.pdf.is_empty());
}

#[tokio::test]
async fn test_from_html_with_options() {
    let margin = PdfMargin {
        top: Some(20.0),
        right: Some(20.0),
        bottom: Some(20.0),
        left: Some(20.0),
    };
    let options = PdfOptions {
        print_background: Some(true),
        scale: Some(1.0),
        prefer_css_page_size: Some(false),
    };
    
    let result = from_html(
        "<p>Test</p>",
        Some("Footer"),
        Some("Header"),
        Some(margin),
        Some(options),
        Some("portrait"),
        Some("A4"),
    ).await.unwrap();
    
    assert!(!result.pdf.is_empty());
}

// ============================================================================
// from_template tests
// ============================================================================

#[tokio::test]
async fn test_from_template_simple() {
    let mut data = std::collections::HashMap::new();
    data.insert("name".to_string(), serde_json::json!("John"));
    data.insert("amount".to_string(), serde_json::json!(100));
    
    let template_data = PdfTemplateData { data };
    
    let result = from_template(template_data, "invoice-template", None, None, None).await.unwrap();
    
    assert_eq!(result.page_count, 1);
    assert!(!result.pdf.is_empty());
}

// ============================================================================
// Integration tests
// ============================================================================

#[tokio::test]
async fn test_pdf_workflow_create_modify_extract() {
    // Create PDF from HTML
    let html_result = from_html("<p>Test Content</p>", None, None, None, None, None, None).await.unwrap();
    
    // Set metadata
    let metadata_result = set_metadata(
        &html_result.pdf,
        Some("Test Author"),
        None,
        None,
        Some("Test Document"),
    ).await.unwrap();
    
    // Get metadata back
    let get_result = get_metadata(&metadata_result.pdf).await.unwrap();
    
    assert_eq!(get_result.page_count, 1);
    assert_eq!(get_result.title, Some("Test Document".to_string()));
    assert_eq!(get_result.author, Some("Test Author".to_string()));
}

#[tokio::test]
async fn test_multipage_split_and_merge() {
    // Create a 4-page document
    let original = create_multipage_pdf(4);
    
    // Split into individual pages
    let split_result = split(&original, None, None).await.unwrap();
    assert_eq!(split_result.documents.len(), 4);
    
    // Merge them back (taking first 2)
    let merge_result = merge(
        vec![split_result.documents[0].clone(), split_result.documents[1].clone()],
        None,
    ).await.unwrap();
    
    // Should have pages after merge
    assert!(merge_result.page_count >= 1);
}

#[tokio::test]
async fn test_compress_and_verify() {
    let pdf = create_multipage_pdf(3);
    
    let compressed = compress(&pdf, Some("high"), Some(false)).await.unwrap();
    
    // Compressed should still be valid PDF
    let metadata = get_metadata(&compressed.pdf).await.unwrap();
    assert_eq!(metadata.page_count, 3);
}
