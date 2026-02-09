use axum::{
    Json,
    body::Body,
    extract::{Multipart, Path, Query, State},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

use crate::error::ServerError;
use crate::extractors::Auth;
use crate::state::AppState;
use harana_components_blob::{BlobInfo, BlobMetadata, BlobStore, ListOptions, PutOptions};

// ============================================================================
// Request / Response types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct DownloadParams {
    /// Optional: override the Content-Disposition filename.
    pub filename: Option<String>,
    /// If true, return as inline instead of attachment.
    #[serde(default)]
    pub inline: bool,
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub prefix: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<u32>,
    pub delimiter: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UploadResponse {
    pub files: Vec<UploadedFile>,
}

#[derive(Debug, Serialize)]
pub struct UploadedFile {
    pub key: String,
    pub size: u64,
    pub content_type: Option<String>,
    pub etag: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DeleteResponse {
    pub deleted: String,
}

#[derive(Debug, Serialize)]
pub struct ListBlobResponse {
    pub objects: Vec<BlobInfo>,
    pub truncated: bool,
    pub cursor: Option<String>,
    pub prefixes: Vec<String>,
}

// ============================================================================
// Helpers
// ============================================================================

/// Build a namespaced blob key scoped to the authenticated user.
fn user_key(user_id: &str, path: &str) -> String {
    let path = path.trim_start_matches('/');
    format!("users/{user_id}/{path}")
}

// ============================================================================
// Handlers
// ============================================================================

/// Upload one or more files via multipart form data.
///
/// `POST /api/blob/upload`
/// `POST /api/blob/upload/*path`
///
/// Each field in the multipart body is stored as a separate blob.
/// The blob key is `users/<user_id>/<path>/<filename>`.
pub async fn upload(
    State(state): State<AppState>,
    auth: Auth,
    path: Option<Path<String>>,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>, ServerError> {
    let blob = state.blob_store()?;
    let prefix = path.map(|p| p.0).unwrap_or_default();
    let mut files = Vec::new();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| ServerError::BadRequest(format!("Multipart error: {e}")))?
    {
        let filename = field
            .file_name()
            .map(|s| s.to_string())
            .or_else(|| field.name().map(|s| s.to_string()))
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let content_type = field
            .content_type()
            .map(|ct| ct.to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string());

        let data = field
            .bytes()
            .await
            .map_err(|e| ServerError::BadRequest(format!("Failed to read field: {e}")))?;

        let key = if prefix.is_empty() {
            user_key(&auth.user_id, &filename)
        } else {
            user_key(&auth.user_id, &format!("{prefix}/{filename}"))
        };

        let options = PutOptions::new().with_metadata(BlobMetadata::new().with_content_type(&content_type));

        let info = blob
            .put(&key, &data, options)
            .await
            .map_err(|e| ServerError::Internal(format!("Blob put failed: {e}")))?;

        files.push(UploadedFile {
            key: info.key,
            size: info.size,
            content_type: info.content_type,
            etag: info.etag,
        });
    }

    if files.is_empty() {
        return Err(ServerError::BadRequest(
            "No files provided in multipart body".to_string(),
        ));
    }

    Ok(Json(UploadResponse { files }))
}

/// Download a file by its path.
///
/// `GET /api/blob/download/*path`
///
/// Returns the raw file bytes with appropriate Content-Type and
/// Content-Disposition headers.
pub async fn download(
    State(state): State<AppState>,
    auth: Auth,
    Path(path): Path<String>,
    Query(params): Query<DownloadParams>,
) -> Result<Response, ServerError> {
    let blob = state.blob_store()?;
    let key = user_key(&auth.user_id, &path);

    let (data, metadata) = blob.get_with_metadata(&key).await.map_err(|e| match e {
        harana_components_blob::BlobError::NotFound(_) => ServerError::NotFound(format!("File not found: {path}")),
        other => ServerError::Internal(format!("Blob get failed: {other}")),
    })?;

    let content_type = metadata
        .content_type
        .unwrap_or_else(|| "application/octet-stream".to_string());

    // Determine the download filename from query params or the last path segment.
    let filename = params
        .filename
        .unwrap_or_else(|| path.rsplit('/').next().unwrap_or("download").to_string());

    let disposition = if params.inline {
        format!("inline; filename=\"{filename}\"")
    } else {
        format!("attachment; filename=\"{filename}\"")
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CONTENT_DISPOSITION, disposition)
        .header(header::CONTENT_LENGTH, data.len())
        .body(Body::from(data))
        .map_err(|e| ServerError::Internal(format!("Failed to build response: {e}")))?)
}

/// Get metadata for a file without downloading.
///
/// `HEAD /api/blob/download/*path`
pub async fn head(
    State(state): State<AppState>,
    auth: Auth,
    Path(path): Path<String>,
) -> Result<Response, ServerError> {
    let blob = state.blob_store()?;
    let key = user_key(&auth.user_id, &path);

    let info = blob.head(&key).await.map_err(|e| match e {
        harana_components_blob::BlobError::NotFound(_) => ServerError::NotFound(format!("File not found: {path}")),
        other => ServerError::Internal(format!("Blob head failed: {other}")),
    })?;

    let content_type = info
        .content_type
        .unwrap_or_else(|| "application/octet-stream".to_string());

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CONTENT_LENGTH, info.size)
        .body(Body::empty())
        .map_err(|e| ServerError::Internal(format!("Failed to build response: {e}")))?)
}

/// Delete a file by its path.
///
/// `DELETE /api/blob/delete/*path`
pub async fn delete(
    State(state): State<AppState>,
    auth: Auth,
    Path(path): Path<String>,
) -> Result<Json<DeleteResponse>, ServerError> {
    let blob = state.blob_store()?;
    let key = user_key(&auth.user_id, &path);

    blob.delete(&key).await.map_err(|e| match e {
        harana_components_blob::BlobError::NotFound(_) => ServerError::NotFound(format!("File not found: {path}")),
        other => ServerError::Internal(format!("Blob delete failed: {other}")),
    })?;

    Ok(Json(DeleteResponse { deleted: key }))
}

/// List files, optionally filtered by prefix.
///
/// `GET /api/blob/list`
pub async fn list(
    State(state): State<AppState>,
    auth: Auth,
    Query(params): Query<ListParams>,
) -> Result<Json<ListBlobResponse>, ServerError> {
    let blob = state.blob_store()?;

    // Scope listing to the user's namespace.
    let user_prefix = format!("users/{}/", auth.user_id);
    let full_prefix = match &params.prefix {
        Some(p) => format!("{user_prefix}{}", p.trim_start_matches('/')),
        None => user_prefix,
    };

    let mut options = ListOptions::new().with_prefix(full_prefix);

    if let Some(cursor) = params.cursor {
        options = options.with_cursor(cursor);
    }
    if let Some(limit) = params.limit {
        options = options.with_limit(limit);
    }
    if let Some(delimiter) = params.delimiter {
        options = options.with_delimiter(delimiter);
    }

    let response = blob
        .list(options)
        .await
        .map_err(|e| ServerError::Internal(format!("Blob list failed: {e}")))?;

    Ok(Json(ListBlobResponse {
        objects: response.objects,
        truncated: response.truncated,
        cursor: response.cursor,
        prefixes: response.prefixes,
    }))
}
