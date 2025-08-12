//! # Kintone File API
//!
//! This module provides functions for interacting with Kintone's file-related REST API endpoints.
//! It includes operations for uploading and downloading files that can be used in file fields
//! or as attachments in Kintone records.

use serde::Deserialize;
use std::io::Read;

use crate::client::{DownloadRequest, KintoneClient, UploadRequest};
use crate::error::ApiError;

/// Uploads a file to Kintone for use in file fields or attachments.
///
/// This function creates a request to upload a file to Kintone's file storage.
/// The uploaded file can then be used in file fields of records or as attachments.
///
/// # Arguments
/// * `filename` - The name of the file to upload
/// * `content` - The file content as a `Read` stream (provided when calling `send()`)
///
/// # Example
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// use std::fs::File;
///
/// let file = File::open("./document.pdf")?;
/// let response = kintone::v1::file::upload("document.pdf").send(&client, file)?;
/// println!("Uploaded file key: {}", response.file_key);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
/// <https://cybozu.dev/ja/kintone/docs/rest-api/files/upload-file/>
pub fn upload(filename: impl Into<String>) -> UploadFileRequest {
    let upload_request =
        UploadRequest::new(http::Method::POST, "/v1/file.json", "file".to_owned(), filename.into());
    UploadFileRequest { upload_request }
}

#[must_use]
pub struct UploadFileRequest {
    upload_request: UploadRequest,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadFileResponse {
    pub file_key: String,
}

impl UploadFileRequest {
    /// Sends the upload request to the Kintone API with file content.
    ///
    /// # Arguments
    /// * `client` - The KintoneClient to use for the API call
    /// * `content` - The file content as a `Read` stream
    ///
    /// # Returns
    /// A Result containing the UploadFileResponse with the file key or an error
    pub fn send(
        self,
        client: &KintoneClient,
        content: impl Read + 'static,
    ) -> Result<UploadFileResponse, ApiError> {
        self.upload_request.send(client, content)
    }
}

//-----------------------------------------------------------------------------

/// Downloads a file from Kintone using its file key.
///
/// This function creates a request to download a file that was previously uploaded
/// to Kintone. The file is identified by its unique file key.
///
/// # Arguments
/// * `file_key` - The unique file key returned from a previous upload operation
///
/// # Example
/// ```no_run
/// # use kintone::client::{Auth, KintoneClient};
/// # let client = KintoneClient::new("https://example.cybozu.com", Auth::password("user".to_owned(), "pass".to_owned()));
/// use std::io::copy;
/// use std::fs::File;
///
/// let mut response = kintone::v1::file::download("file_key_from_upload")
///     .send(&client)?;
///
/// let mut output_file = File::create("./downloaded_file.pdf")?;
/// copy(&mut response.content, &mut output_file)?;
/// println!("Downloaded file with MIME type: {}", response.mime_type);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// # Reference
/// <https://cybozu.dev/ja/kintone/docs/rest-api/files/download-file/>
pub fn download(file_key: impl Into<String>) -> DownloadFileRequest {
    let download_request =
        DownloadRequest::new(http::Method::GET, "/v1/file.json").query("fileKey", file_key.into());
    DownloadFileRequest { download_request }
}

#[must_use]
pub struct DownloadFileRequest {
    download_request: DownloadRequest,
}

impl DownloadFileRequest {
    pub fn send(self, client: &KintoneClient) -> Result<DownloadFileResponse, ApiError> {
        let resp = self.download_request.send(client)?;
        Ok(DownloadFileResponse {
            mime_type: resp.mime_type,
            content: resp.content,
        })
    }
}

/// Response containing downloaded file data from Kintone.
///
/// This struct contains the file content as a readable stream and the MIME type
/// of the downloaded file. The content can be read or copied to a file or other destination.
///
/// # Fields
/// * `mime_type` - The MIME type of the downloaded file (e.g., "application/pdf", "image/jpeg")
/// * `content` - A readable stream containing the file data
pub struct DownloadFileResponse {
    pub mime_type: String,
    pub content: Box<dyn Read + Send + Sync + 'static>,
}

impl std::fmt::Debug for DownloadFileResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DownloadFileResponse")
            .field("mime_type", &self.mime_type)
            .finish()
    }
}
