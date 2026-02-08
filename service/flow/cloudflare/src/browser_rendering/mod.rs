// Harana Actions - Cloudflare Browser Rendering Module
// This module provides Cloudflare Browser Rendering actions for rendering pages,
// taking screenshots, extracting content, and generating PDFs.

pub mod output;

use output::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use worker::Env;

fn js_err(e: JsValue) -> String {
    format!("Browser Rendering JS error: {e:?}")
}

/// Get the browser rendering binding from the environment
fn get_binding(env: &Env, binding: &str) -> Result<JsValue, String> {
    let val = js_sys::Reflect::get(env, &JsValue::from(binding))
        .map_err(|_| format!("Browser Rendering binding '{binding}' not found"))?;
    if val.is_undefined() {
        return Err(format!("Browser Rendering binding '{binding}' is undefined"));
    }
    Ok(val)
}

/// Render Page In Browser
pub async fn render(env: &Env, binding: &str, url: &str) -> Result<RenderOutput, String> {
    let browser_binding = get_binding(env, binding)?;

    let opts = js_sys::Object::new();
    js_sys::Reflect::set(&opts, &JsValue::from("url"), &JsValue::from(url)).map_err(js_err)?;

    let fetch_fn = js_sys::Reflect::get(&browser_binding, &JsValue::from("fetch")).map_err(js_err)?;
    let fetch_fn: js_sys::Function = fetch_fn.dyn_into().map_err(|_| "fetch is not a function".to_string())?;

    let promise = fetch_fn.call1(&browser_binding, &JsValue::from(url)).map_err(js_err)?;

    let response = JsFuture::from(js_sys::Promise::from(promise)).await.map_err(js_err)?;

    let text_fn = js_sys::Reflect::get(&response, &JsValue::from("text")).map_err(js_err)?;
    let text_fn: js_sys::Function = text_fn.dyn_into().map_err(|_| "text is not a function".to_string())?;

    let text_promise = text_fn.call0(&response).map_err(js_err)?;
    let content = JsFuture::from(js_sys::Promise::from(text_promise))
        .await
        .map_err(js_err)?;

    let status_code = js_sys::Reflect::get(&response, &JsValue::from("status"))
        .map_err(js_err)?
        .as_f64()
        .unwrap_or(200.0) as i32;

    Ok(RenderOutput {
        content: content.as_string().unwrap_or_default(),
        status_code,
    })
}

/// Take Browser Screenshot
pub async fn screenshot(
    env: &Env,
    binding: &str,
    url: &str,
    full_page: Option<bool>,
    height: Option<i32>,
    width: Option<i32>,
    r#type: Option<&str>,
) -> Result<ScreenshotOutput, String> {
    let browser_binding = get_binding(env, binding)?;

    let opts = js_sys::Object::new();
    js_sys::Reflect::set(&opts, &JsValue::from("url"), &JsValue::from(url)).map_err(js_err)?;
    if let Some(fp) = full_page {
        js_sys::Reflect::set(&opts, &JsValue::from("fullPage"), &JsValue::from(fp)).map_err(js_err)?;
    }
    if let Some(h) = height {
        js_sys::Reflect::set(&opts, &JsValue::from("height"), &JsValue::from(h)).map_err(js_err)?;
    }
    if let Some(w) = width {
        js_sys::Reflect::set(&opts, &JsValue::from("width"), &JsValue::from(w)).map_err(js_err)?;
    }
    if let Some(t) = r#type {
        js_sys::Reflect::set(&opts, &JsValue::from("type"), &JsValue::from(t)).map_err(js_err)?;
    }

    let screenshot_fn = js_sys::Reflect::get(&browser_binding, &JsValue::from("screenshot")).map_err(js_err)?;
    let screenshot_fn: js_sys::Function = screenshot_fn
        .dyn_into()
        .map_err(|_| "screenshot is not a function".to_string())?;

    let promise = screenshot_fn.call1(&browser_binding, &opts.into()).map_err(js_err)?;

    let result = JsFuture::from(js_sys::Promise::from(promise)).await.map_err(js_err)?;

    let array = js_sys::Uint8Array::new(&result);
    Ok(ScreenshotOutput { image: array.to_vec() })
}

/// Extract Browser Page Content
pub async fn extract_content(
    env: &Env,
    binding: &str,
    url: &str,
    selector: Option<&str>,
) -> Result<ExtractContentOutput, String> {
    let browser_binding = get_binding(env, binding)?;

    let opts = js_sys::Object::new();
    js_sys::Reflect::set(&opts, &JsValue::from("url"), &JsValue::from(url)).map_err(js_err)?;
    if let Some(sel) = selector {
        js_sys::Reflect::set(&opts, &JsValue::from("selector"), &JsValue::from(sel)).map_err(js_err)?;
    }

    let extract_fn = js_sys::Reflect::get(&browser_binding, &JsValue::from("extractContent")).map_err(js_err)?;
    let extract_fn: js_sys::Function = extract_fn
        .dyn_into()
        .map_err(|_| "extractContent is not a function".to_string())?;

    let promise = extract_fn.call1(&browser_binding, &opts.into()).map_err(js_err)?;

    let result = JsFuture::from(js_sys::Promise::from(promise)).await.map_err(js_err)?;

    let content = js_sys::Reflect::get(&result, &JsValue::from("content"))
        .map_err(js_err)?
        .as_string()
        .unwrap_or_default();
    let title = js_sys::Reflect::get(&result, &JsValue::from("title"))
        .map_err(js_err)?
        .as_string()
        .unwrap_or_default();

    Ok(ExtractContentOutput { content, title })
}

/// Generate Browser Page PDF
pub async fn pdf(
    env: &Env,
    binding: &str,
    url: &str,
    format: Option<&str>,
    landscape: Option<bool>,
    print_background: Option<bool>,
) -> Result<PdfOutput, String> {
    let browser_binding = get_binding(env, binding)?;

    let opts = js_sys::Object::new();
    js_sys::Reflect::set(&opts, &JsValue::from("url"), &JsValue::from(url)).map_err(js_err)?;
    if let Some(f) = format {
        js_sys::Reflect::set(&opts, &JsValue::from("format"), &JsValue::from(f)).map_err(js_err)?;
    }
    if let Some(l) = landscape {
        js_sys::Reflect::set(&opts, &JsValue::from("landscape"), &JsValue::from(l)).map_err(js_err)?;
    }
    if let Some(pb) = print_background {
        js_sys::Reflect::set(&opts, &JsValue::from("printBackground"), &JsValue::from(pb)).map_err(js_err)?;
    }

    let pdf_fn = js_sys::Reflect::get(&browser_binding, &JsValue::from("pdf")).map_err(js_err)?;
    let pdf_fn: js_sys::Function = pdf_fn.dyn_into().map_err(|_| "pdf is not a function".to_string())?;

    let promise = pdf_fn.call1(&browser_binding, &opts.into()).map_err(js_err)?;

    let result = JsFuture::from(js_sys::Promise::from(promise)).await.map_err(js_err)?;

    let array = js_sys::Uint8Array::new(&result);
    Ok(PdfOutput { pdf: array.to_vec() })
}
