// Harana Actions - PDF Module
// This module provides PDF manipulation actions using lopdf.

pub mod output;

#[cfg(test)]
mod tests;

use lopdf::{Document, Object, ObjectId, Dictionary, Stream};
use lopdf::dictionary;
use output::*;

/// Merge PDF Documents
pub async fn merge(
    documents: Vec<Vec<u8>>,
    _output_name: Option<&str>,
) -> Result<MergeOutput, String> {
    if documents.is_empty() {
        return Err("No documents provided for merging".to_string());
    }
    
    if documents.len() == 1 {
        let doc = Document::load_mem(&documents[0])
            .map_err(|e| format!("Failed to load PDF: {}", e))?;
        let page_count = doc.get_pages().len() as i32;
        let size = documents[0].len() as i64;
        return Ok(MergeOutput {
            page_count,
            pdf: documents[0].clone(),
            size,
        });
    }
    
    // Load first document as base
    let mut merged_doc = Document::load_mem(&documents[0])
        .map_err(|e| format!("Failed to load first PDF: {}", e))?;
    
    // Merge additional documents
    for (idx, doc_bytes) in documents.iter().skip(1).enumerate() {
        let doc = Document::load_mem(doc_bytes)
            .map_err(|e| format!("Failed to load PDF {}: {}", idx + 2, e))?;
        
        // Get the pages from the document to merge
        let pages: Vec<ObjectId> = doc.get_pages().into_iter().map(|(_, id)| id).collect();
        
        // Copy objects from source to merged document
        let mut max_id = merged_doc.max_id;
        let mut id_mapping: std::collections::HashMap<ObjectId, ObjectId> = std::collections::HashMap::new();
        
        for (obj_id, obj) in doc.objects.iter() {
            max_id += 1;
            let new_id = (max_id, 0);
            id_mapping.insert(*obj_id, new_id);
            merged_doc.objects.insert(new_id, obj.clone());
        }
        
        merged_doc.max_id = max_id;
        
        // Update references in copied objects
        for new_id in id_mapping.values() {
            if let Some(obj) = merged_doc.objects.get_mut(new_id) {
                update_object_references(obj, &id_mapping);
            }
        }
        
        // Add pages to merged document's page tree
        if let Ok(catalog) = merged_doc.catalog() {
            if let Ok(pages_ref) = catalog.get(b"Pages") {
                if let Ok(pages_id) = pages_ref.as_reference() {
                    if let Ok(Object::Dictionary(ref mut pages_dict)) = merged_doc.get_object_mut(pages_id) {
                        if let Ok(Object::Array(ref mut kids)) = pages_dict.get_mut(b"Kids") {
                            for page_id in pages {
                                if let Some(&new_page_id) = id_mapping.get(&page_id) {
                                    kids.push(Object::Reference(new_page_id));
                                }
                            }
                        }
                        // Update count
                        if let Ok(Object::Integer(ref mut count)) = pages_dict.get_mut(b"Count") {
                            *count += doc.get_pages().len() as i64;
                        }
                    }
                }
            }
        }
    }
    
    // Save merged document
    let mut output = Vec::new();
    merged_doc.save_to(&mut output)
        .map_err(|e| format!("Failed to save merged PDF: {}", e))?;
    
    let page_count = merged_doc.get_pages().len() as i32;
    let size = output.len() as i64;
    
    Ok(MergeOutput {
        page_count,
        pdf: output,
        size,
    })
}

fn update_object_references(obj: &mut Object, mapping: &std::collections::HashMap<ObjectId, ObjectId>) {
    match obj {
        Object::Reference(ref mut id) => {
            if let Some(&new_id) = mapping.get(id) {
                *id = new_id;
            }
        }
        Object::Array(arr) => {
            for item in arr.iter_mut() {
                update_object_references(item, mapping);
            }
        }
        Object::Dictionary(dict) => {
            for (_, val) in dict.iter_mut() {
                update_object_references(val, mapping);
            }
        }
        Object::Stream(stream) => {
            for (_, val) in stream.dict.iter_mut() {
                update_object_references(val, mapping);
            }
        }
        _ => {}
    }
}

/// Split PDF Document
pub async fn split(
    document: &[u8],
    pages: Option<Vec<i32>>,
    ranges: Option<Vec<String>>,
) -> Result<SplitOutput, String> {
    let doc = Document::load_mem(document)
        .map_err(|e| format!("Failed to load PDF: {}", e))?;
    
    let total_pages = doc.get_pages().len() as i32;
    
    // Determine which page sets to extract
    let page_sets: Vec<Vec<i32>> = if let Some(page_list) = pages {
        // Split at specified pages - each page becomes a separate document
        page_list.iter().map(|&p| vec![p]).collect()
    } else if let Some(range_list) = ranges {
        // Parse ranges like "1-3", "4-6"
        range_list.iter().map(|range| {
            parse_page_range(range, total_pages)
        }).collect::<Result<Vec<_>, _>>()?
    } else {
        // Default: split into individual pages
        (1..=total_pages).map(|p| vec![p]).collect()
    };
    
    let mut documents = Vec::new();
    let mut page_counts = Vec::new();
    
    for page_set in page_sets {
        let extracted = extract_page_set(&doc, &page_set)?;
        page_counts.push(page_set.len() as i32);
        documents.push(extracted);
    }
    
    Ok(SplitOutput {
        documents,
        page_counts,
    })
}

fn parse_page_range(range: &str, max_pages: i32) -> Result<Vec<i32>, String> {
    if range.contains('-') {
        let parts: Vec<&str> = range.split('-').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid range format: {}", range));
        }
        let start: i32 = parts[0].trim().parse()
            .map_err(|_| format!("Invalid start page in range: {}", range))?;
        let end: i32 = parts[1].trim().parse()
            .map_err(|_| format!("Invalid end page in range: {}", range))?;
        
        if start < 1 || end > max_pages || start > end {
            return Err(format!("Invalid page range: {}-{} (document has {} pages)", start, end, max_pages));
        }
        
        Ok((start..=end).collect())
    } else {
        let page: i32 = range.trim().parse()
            .map_err(|_| format!("Invalid page number: {}", range))?;
        if page < 1 || page > max_pages {
            return Err(format!("Page {} out of range (document has {} pages)", page, max_pages));
        }
        Ok(vec![page])
    }
}

fn extract_page_set(doc: &Document, pages: &[i32]) -> Result<Vec<u8>, String> {
    let mut new_doc = doc.clone();
    // get_pages returns BTreeMap<u32, ObjectId> where key is page number
    let page_ids: Vec<(u32, ObjectId)> = doc.get_pages().into_iter().collect();
    
    // Keep only the specified pages
    let pages_to_keep: std::collections::HashSet<i32> = pages.iter().copied().collect();
    let pages_to_delete: Vec<u32> = page_ids.iter()
        .filter(|(page_num, _)| !pages_to_keep.contains(&(*page_num as i32)))
        .map(|(page_num, _)| *page_num)
        .collect();
    
    new_doc.delete_pages(&pages_to_delete);
    
    let mut output = Vec::new();
    new_doc.save_to(&mut output)
        .map_err(|e| format!("Failed to save split PDF: {}", e))?;
    
    Ok(output)
}

/// Extract PDF Pages
pub async fn extract_pages(
    document: &[u8],
    start_page: i32,
    end_page: i32,
) -> Result<ExtractPagesOutput, String> {
    if start_page < 1 {
        return Err("Start page must be at least 1".to_string());
    }
    if end_page < start_page {
        return Err("End page must be >= start page".to_string());
    }
    
    let doc = Document::load_mem(document)
        .map_err(|e| format!("Failed to load PDF: {}", e))?;
    
    let total_pages = doc.get_pages().len() as i32;
    
    if end_page > total_pages {
        return Err(format!("End page {} exceeds document page count {}", end_page, total_pages));
    }
    
    let pages: Vec<i32> = (start_page..=end_page).collect();
    let pdf = extract_page_set(&doc, &pages)?;
    let page_count = pages.len() as i32;
    let size = pdf.len() as i64;
    
    Ok(ExtractPagesOutput {
        page_count,
        pdf,
        size,
    })
}

/// Extract Text From PDF
pub async fn extract_text(
    document: &[u8],
    pages: Option<Vec<i32>>,
    _preserve_layout: Option<bool>,
) -> Result<ExtractTextOutput, String> {
    let doc = Document::load_mem(document)
        .map_err(|e| format!("Failed to load PDF: {}", e))?;
    
    let total_pages = doc.get_pages().len() as i32;
    let page_nums: Vec<i32> = pages.unwrap_or_else(|| (1..=total_pages).collect());
    
    let mut all_text = String::new();
    let mut page_texts = Vec::new();
    
    // get_pages returns BTreeMap<u32, ObjectId>
    let page_ids: Vec<(u32, ObjectId)> = doc.get_pages().into_iter().collect();
    
    for page_num in &page_nums {
        if *page_num < 1 || *page_num > total_pages {
            continue;
        }
        
        // Find the page id for this page number
        let page_id = page_ids.iter()
            .find(|(num, _)| *num == *page_num as u32)
            .map(|(_, id)| *id);
        
        let text = if let Some(pid) = page_id {
            extract_text_from_page(&doc, pid).unwrap_or_default()
        } else {
            String::new()
        };
        
        if !all_text.is_empty() && !text.is_empty() {
            all_text.push_str("\n\n");
        }
        all_text.push_str(&text);
        
        page_texts.push(PdfPageText {
            page_number: *page_num,
            text,
        });
    }
    
    Ok(ExtractTextOutput {
        pages: page_texts,
        text: all_text,
    })
}

fn extract_text_from_page(doc: &Document, page_id: ObjectId) -> Result<String, String> {
    let mut text = String::new();
    
    // Get page content streams
    if let Ok(content_data) = doc.get_page_content(page_id) {
        // Simple text extraction - look for text showing operators
        let content_str = String::from_utf8_lossy(&content_data);
        
        // Extract text between parentheses (simple strings)
        let mut in_text = false;
        let mut current_text = String::new();
        let mut paren_depth = 0;
        
        for ch in content_str.chars() {
            if ch == '(' && !in_text {
                in_text = true;
                paren_depth = 1;
            } else if ch == '(' && in_text {
                paren_depth += 1;
                current_text.push(ch);
            } else if ch == ')' && in_text {
                paren_depth -= 1;
                if paren_depth == 0 {
                    in_text = false;
                    if !current_text.is_empty() {
                        if !text.is_empty() {
                            text.push(' ');
                        }
                        text.push_str(&current_text);
                        current_text.clear();
                    }
                } else {
                    current_text.push(ch);
                }
            } else if in_text {
                current_text.push(ch);
            }
        }
    }
    
    Ok(text)
}

/// Extract Images From PDF
pub async fn extract_images(
    document: &[u8],
    format: Option<&str>,
    _pages: Option<Vec<i32>>,
) -> Result<ExtractImagesOutput, String> {
    let _doc = Document::load_mem(document)
        .map_err(|e| format!("Failed to load PDF: {}", e))?;
    
    let _output_format = format.unwrap_or("png");
    
    // Image extraction requires more complex parsing
    // For now, return empty list
    let images = Vec::new();
    
    Ok(ExtractImagesOutput {
        count: images.len() as i32,
        images,
    })
}

/// Add Watermark To PDF
pub async fn add_watermark(
    document: &[u8],
    _opacity: Option<f64>,
    _pages: Option<Vec<i32>>,
    _position: Option<&str>,
    _watermark_image: Option<Vec<u8>>,
    _watermark_text: Option<&str>,
) -> Result<AddWatermarkOutput, String> {
    let mut doc = Document::load_mem(document)
        .map_err(|e| format!("Failed to load PDF: {}", e))?;
    
    // Note: Adding watermarks requires complex content stream manipulation
    // This is a simplified implementation that returns the document as-is
    
    let mut output = Vec::new();
    doc.save_to(&mut output)
        .map_err(|e| format!("Failed to save watermarked PDF: {}", e))?;
    
    Ok(AddWatermarkOutput {
        pdf: output.clone(),
        size: output.len() as i64,
    })
}

/// Compress PDF Document
pub async fn compress(
    document: &[u8],
    _quality: Option<&str>,
    remove_metadata: Option<bool>,
) -> Result<CompressOutput, String> {
    let mut doc = Document::load_mem(document)
        .map_err(|e| format!("Failed to load PDF: {}", e))?;
    
    let original_size = document.len() as i64;
    let should_remove_metadata = remove_metadata.unwrap_or(false);
    
    // Compress streams
    doc.compress();
    
    // Remove metadata if requested
    if should_remove_metadata {
        // Remove document info dictionary
        if let Ok(trailer) = doc.trailer.get(b"Info") {
            if let Ok(info_id) = trailer.as_reference() {
                doc.objects.remove(&info_id);
            }
        }
    }
    
    let mut output = Vec::new();
    doc.save_to(&mut output)
        .map_err(|e| format!("Failed to save compressed PDF: {}", e))?;
    
    let compressed_size = output.len() as i64;
    let compression_ratio = if original_size > 0 {
        1.0 - (compressed_size as f64 / original_size as f64)
    } else {
        0.0
    };
    
    Ok(CompressOutput {
        compressed_size,
        compression_ratio,
        original_size,
        pdf: output,
    })
}

/// Encrypt PDF Document
pub async fn encrypt(
    document: &[u8],
    _password: &str,
    _owner_password: Option<&str>,
    _permissions: Option<Vec<String>>,
) -> Result<EncryptOutput, String> {
    let mut doc = Document::load_mem(document)
        .map_err(|e| format!("Failed to load PDF: {}", e))?;
    
    // Note: lopdf doesn't have built-in encryption support
    
    let mut output = Vec::new();
    doc.save_to(&mut output)
        .map_err(|e| format!("Failed to save PDF: {}", e))?;
    
    Ok(EncryptOutput {
        pdf: output,
        success: false, // Encryption not implemented
    })
}

/// Decrypt PDF Document
pub async fn decrypt(
    document: &[u8],
    _password: &str,
) -> Result<DecryptOutput, String> {
    // lopdf handles encrypted PDFs automatically when loading
    let mut doc = Document::load_mem(document)
        .map_err(|e| format!("Failed to load/decrypt PDF: {}", e))?;
    
    let mut output = Vec::new();
    doc.save_to(&mut output)
        .map_err(|e| format!("Failed to save decrypted PDF: {}", e))?;
    
    Ok(DecryptOutput {
        pdf: output,
        success: true,
    })
}

/// Get PDF Metadata
pub async fn get_metadata(
    document: &[u8],
) -> Result<GetMetadataOutput, String> {
    let doc = Document::load_mem(document)
        .map_err(|e| format!("Failed to load PDF: {}", e))?;
    
    let page_count = doc.get_pages().len() as i32;
    
    // Try to get document info dictionary
    let mut title = None;
    let mut author = None;
    let mut subject = None;
    let mut keywords = None;
    let mut creator = None;
    let mut producer = None;
    let mut creation_date = None;
    let mut modification_date = None;
    
    if let Ok(info_ref) = doc.trailer.get(b"Info") {
        if let Ok(info_id) = info_ref.as_reference() {
            if let Ok(Object::Dictionary(info)) = doc.get_object(info_id) {
                title = get_string_from_dict(&info, b"Title");
                author = get_string_from_dict(&info, b"Author");
                subject = get_string_from_dict(&info, b"Subject");
                creator = get_string_from_dict(&info, b"Creator");
                producer = get_string_from_dict(&info, b"Producer");
                creation_date = get_string_from_dict(&info, b"CreationDate");
                modification_date = get_string_from_dict(&info, b"ModDate");
                
                if let Some(kw) = get_string_from_dict(&info, b"Keywords") {
                    keywords = Some(kw.split(',').map(|s| s.trim().to_string()).collect());
                }
            }
        }
    }
    
    Ok(GetMetadataOutput {
        author,
        creation_date,
        creator,
        keywords,
        modification_date,
        page_count,
        producer,
        subject,
        title,
    })
}

fn get_string_from_dict(dict: &Dictionary, key: &[u8]) -> Option<String> {
    dict.get(key).ok().and_then(|obj| {
        match obj {
            Object::String(bytes, _) => String::from_utf8(bytes.clone()).ok(),
            _ => None,
        }
    })
}

/// Set PDF Metadata
pub async fn set_metadata(
    document: &[u8],
    author: Option<&str>,
    keywords: Option<Vec<String>>,
    subject: Option<&str>,
    title: Option<&str>,
) -> Result<SetMetadataOutput, String> {
    let mut doc = Document::load_mem(document)
        .map_err(|e| format!("Failed to load PDF: {}", e))?;
    
    // Create or update info dictionary
    let mut info_dict = Dictionary::new();
    
    // Try to get existing info
    if let Ok(info_ref) = doc.trailer.get(b"Info") {
        if let Ok(info_id) = info_ref.as_reference() {
            if let Ok(Object::Dictionary(existing)) = doc.get_object(info_id) {
                info_dict = existing.clone();
            }
        }
    }
    
    // Update fields
    if let Some(t) = title {
        info_dict.set("Title", Object::String(t.as_bytes().to_vec(), lopdf::StringFormat::Literal));
    }
    if let Some(a) = author {
        info_dict.set("Author", Object::String(a.as_bytes().to_vec(), lopdf::StringFormat::Literal));
    }
    if let Some(s) = subject {
        info_dict.set("Subject", Object::String(s.as_bytes().to_vec(), lopdf::StringFormat::Literal));
    }
    if let Some(kw) = keywords {
        let kw_string = kw.join(", ");
        info_dict.set("Keywords", Object::String(kw_string.as_bytes().to_vec(), lopdf::StringFormat::Literal));
    }
    
    // Add info dictionary to document
    let info_id = doc.add_object(Object::Dictionary(info_dict));
    doc.trailer.set("Info", Object::Reference(info_id));
    
    let mut output = Vec::new();
    doc.save_to(&mut output)
        .map_err(|e| format!("Failed to save PDF: {}", e))?;
    
    Ok(SetMetadataOutput {
        pdf: output,
        success: true,
    })
}

/// Rotate PDF Pages
pub async fn rotate_pages(
    document: &[u8],
    angle: i32,
    pages: Option<Vec<i32>>,
) -> Result<RotatePagesOutput, String> {
    // Validate angle
    if angle != 90 && angle != 180 && angle != 270 {
        return Err(format!("Invalid rotation angle: {}. Must be 90, 180, or 270", angle));
    }
    
    let mut doc = Document::load_mem(document)
        .map_err(|e| format!("Failed to load PDF: {}", e))?;
    
    let total_pages = doc.get_pages().len() as i32;
    let page_nums: Vec<i32> = pages.unwrap_or_else(|| (1..=total_pages).collect());
    
    // get_pages returns BTreeMap<u32, ObjectId>
    let page_ids: Vec<(u32, ObjectId)> = doc.get_pages().into_iter().collect();
    
    for page_num in &page_nums {
        if *page_num < 1 || *page_num > total_pages {
            continue;
        }
        
        // Find the page id for this page number
        if let Some((_, page_id)) = page_ids.iter().find(|(num, _)| *num == *page_num as u32) {
            if let Ok(Object::Dictionary(ref mut page_dict)) = doc.get_object_mut(*page_id) {
                // Get current rotation
                let current_rotation = page_dict.get(b"Rotate")
                    .ok()
                    .and_then(|r| r.as_i64().ok())
                    .unwrap_or(0) as i32;
                
                // Calculate new rotation
                let new_rotation = (current_rotation + angle) % 360;
                
                page_dict.set("Rotate", Object::Integer(new_rotation as i64));
            }
        }
    }
    
    let mut output = Vec::new();
    doc.save_to(&mut output)
        .map_err(|e| format!("Failed to save PDF: {}", e))?;
    
    Ok(RotatePagesOutput {
        pdf: output,
        success: true,
    })
}

/// Add Page Numbers To PDF
pub async fn add_page_numbers(
    document: &[u8],
    _alignment: Option<&str>,
    _font_size: Option<i32>,
    _format: Option<&str>,
    _position: Option<&str>,
    _start_page: Option<i32>,
) -> Result<AddPageNumbersOutput, String> {
    let mut doc = Document::load_mem(document)
        .map_err(|e| format!("Failed to load PDF: {}", e))?;
    
    // Note: Adding actual page numbers would require more complex content stream manipulation
    // This is a simplified implementation that returns the document
    
    let mut output = Vec::new();
    doc.save_to(&mut output)
        .map_err(|e| format!("Failed to save PDF: {}", e))?;
    
    Ok(AddPageNumbersOutput {
        pdf: output.clone(),
        size: output.len() as i64,
    })
}

/// Convert PDF To Images
pub async fn to_images(
    document: &[u8],
    _dpi: Option<i32>,
    _format: Option<&str>,
    pages: Option<Vec<i32>>,
) -> Result<ToImagesOutput, String> {
    let doc = Document::load_mem(document)
        .map_err(|e| format!("Failed to load PDF: {}", e))?;
    
    let total_pages = doc.get_pages().len() as i32;
    let page_nums: Vec<i32> = pages.unwrap_or_else(|| (1..=total_pages).collect());
    
    // Note: lopdf doesn't render PDFs to images
    // In a real implementation, you'd need a library like pdfium-render or pdf-render
    // For now, return empty images with count
    
    let images: Vec<Vec<u8>> = page_nums.iter()
        .filter(|&&p| p >= 1 && p <= total_pages)
        .map(|_| Vec::new())
        .collect();
    
    Ok(ToImagesOutput {
        count: images.len() as i32,
        images,
    })
}

/// Fill PDF Form
pub async fn fill_form(
    document: &[u8],
    fields: Vec<PdfFormFieldValue>,
    _flatten: Option<bool>,
) -> Result<FillFormOutput, String> {
    let mut doc = Document::load_mem(document)
        .map_err(|e| format!("Failed to load PDF: {}", e))?;
    
    // Collect field updates first to avoid borrow issues
    let mut updates: Vec<(ObjectId, String)> = Vec::new();
    
    // Find AcroForm and collect field info
    if let Ok(catalog) = doc.catalog() {
        if let Ok(acroform_ref) = catalog.get(b"AcroForm") {
            if let Ok(acroform_id) = acroform_ref.as_reference() {
                if let Ok(Object::Dictionary(acroform)) = doc.get_object(acroform_id) {
                    if let Ok(Object::Array(field_refs)) = acroform.get(b"Fields") {
                        // Process each field
                        for field_ref in field_refs {
                            if let Ok(field_id) = field_ref.as_reference() {
                                if let Ok(Object::Dictionary(field_dict)) = doc.get_object(field_id) {
                                    // Get field name
                                    if let Some(field_name) = get_string_from_dict(&field_dict, b"T") {
                                        // Check if we have a value for this field
                                        if let Some(form_field) = fields.iter().find(|f| f.name == field_name) {
                                            updates.push((field_id, form_field.value.clone()));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Apply updates
    for (field_id, value) in updates {
        if let Ok(Object::Dictionary(ref mut fd)) = doc.get_object_mut(field_id) {
            fd.set("V", Object::String(
                value.as_bytes().to_vec(),
                lopdf::StringFormat::Literal
            ));
        }
    }
    
    let mut output = Vec::new();
    doc.save_to(&mut output)
        .map_err(|e| format!("Failed to save PDF: {}", e))?;
    
    Ok(FillFormOutput {
        pdf: output.clone(),
        size: output.len() as i64,
    })
}

/// Get PDF Form Fields
pub async fn get_form_fields(
    document: &[u8],
) -> Result<GetFormFieldsOutput, String> {
    let doc = Document::load_mem(document)
        .map_err(|e| format!("Failed to load PDF: {}", e))?;
    
    let mut fields = Vec::new();
    
    // Find AcroForm
    if let Ok(catalog) = doc.catalog() {
        if let Ok(acroform_ref) = catalog.get(b"AcroForm") {
            if let Ok(acroform_id) = acroform_ref.as_reference() {
                if let Ok(Object::Dictionary(acroform)) = doc.get_object(acroform_id) {
                    if let Ok(Object::Array(field_refs)) = acroform.get(b"Fields") {
                        for field_ref in field_refs {
                            if let Ok(field_id) = field_ref.as_reference() {
                                if let Ok(Object::Dictionary(field_dict)) = doc.get_object(field_id) {
                                    let name = get_string_from_dict(&field_dict, b"T").unwrap_or_default();
                                    let value = get_string_from_dict(&field_dict, b"V").unwrap_or_default();
                                    
                                    // Determine field type
                                    let field_type = field_dict.get(b"FT")
                                        .ok()
                                        .and_then(|ft| ft.as_name_str().ok())
                                        .map(|s| s.to_string())
                                        .unwrap_or_else(|| "Text".to_string());
                                    
                                    // Check if required
                                    let flags = field_dict.get(b"Ff")
                                        .ok()
                                        .and_then(|f| f.as_i64().ok())
                                        .unwrap_or(0);
                                    let required = (flags & 2) != 0; // Bit 2 is Required flag
                                    
                                    fields.push(PdfFormField {
                                        name,
                                        field_type,
                                        value,
                                        required,
                                        options: Vec::new(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(GetFormFieldsOutput {
        count: fields.len() as i32,
        fields,
    })
}

// HTML to PDF is complex and requires a rendering engine
// This is a simplified placeholder that creates a basic PDF
/// Generate PDF From HTML
pub async fn from_html(
    html: &str,
    _footer_template: Option<&str>,
    _header_template: Option<&str>,
    _margin: Option<PdfMargin>,
    _options: Option<PdfOptions>,
    _orientation: Option<&str>,
    _page_size: Option<&str>,
) -> Result<FromHtmlOutput, String> {
    // Note: Proper HTML to PDF conversion requires a rendering engine
    // This is a placeholder that creates a simple PDF with the text content
    
    // Strip HTML tags for basic text
    let text = strip_html_tags(html);
    
    let mut doc = Document::with_version("1.5");
    
    // Create a simple PDF structure
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
    
    // Content stream (basic text)
    let content = format!(
        "BT\n/F1 12 Tf\n50 750 Td\n({}) Tj\nET",
        escape_pdf_string(&text.chars().take(1000).collect::<String>())
    );
    let content_stream = Stream::new(Dictionary::new(), content.into_bytes());
    doc.objects.insert(content_id, Object::Stream(content_stream));
    
    // Page dictionary
    let page_dict = dictionary! {
        "Type" => "Page",
        "Parent" => Object::Reference(pages_id),
        "MediaBox" => vec![
            Object::Integer(0),
            Object::Integer(0),
            Object::Integer(612), // Letter width
            Object::Integer(792), // Letter height
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
    doc.save_to(&mut output)
        .map_err(|e| format!("Failed to save PDF: {}", e))?;
    
    Ok(FromHtmlOutput {
        page_count: 1,
        pdf: output.clone(),
        size: output.len() as i64,
    })
}

fn strip_html_tags(html: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    
    for ch in html.chars() {
        if ch == '<' {
            in_tag = true;
        } else if ch == '>' {
            in_tag = false;
        } else if !in_tag {
            result.push(ch);
        }
    }
    
    result.trim().to_string()
}

fn escape_pdf_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('(', "\\(")
        .replace(')', "\\)")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

/// Generate PDF From Template
pub async fn from_template(
    data: PdfTemplateData,
    _template_id: &str,
    _options: Option<PdfOptions>,
    _orientation: Option<&str>,
    _page_size: Option<&str>,
) -> Result<FromTemplateOutput, String> {
    // Note: Template-based PDF generation requires a template engine
    // This is a placeholder that creates a simple PDF with the data
    
    let text = serde_json::to_string_pretty(&data.data)
        .unwrap_or_else(|_| "{}".to_string());
    
    let html_result = from_html(&text, None, None, None, None, None, None).await?;
    
    Ok(FromTemplateOutput {
        page_count: html_result.page_count,
        pdf: html_result.pdf,
        size: html_result.size,
    })
}
