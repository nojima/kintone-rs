use std::io::Read;
use serde::Deserialize;

use crate::ApiResult;
use crate::client::{KintoneClient, UploadRequest, DownloadRequest};

// https://cybozu.dev/ja/kintone/docs/rest-api/files/upload-file/
pub fn upload(filename: String) -> UploadFileRequest {
    let upload_request = UploadRequest::new(
        http::Method::POST,
        "/v1/file.json",
        "file".to_string(),
        filename,
    );
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
    pub fn send(self, client: &KintoneClient, content: impl Read) -> ApiResult<UploadFileResponse> {
        let resp = self.upload_request.send(client, content)?;
        Ok(resp.into_json()?)
    }
}

//-----------------------------------------------------------------------------

// https://cybozu.dev/ja/kintone/docs/rest-api/files/download-file/
pub fn download(file_key: String) -> DownloadFileRequest {
    let download_request = DownloadRequest::new(http::Method::GET, "/v1/file.json")
        .query("fileKey", file_key);
    DownloadFileRequest { download_request }
}

#[must_use]
pub struct DownloadFileRequest {
    download_request: DownloadRequest,
}

impl DownloadFileRequest {
    pub fn send(self, client: &KintoneClient) -> ApiResult<DownloadFileResponse> {
        let resp = self.download_request.send(client)?;
        Ok(DownloadFileResponse {
            mime_type: resp.mime_type,
            content: resp.content,
        })
    }
}

pub struct DownloadFileResponse {
    pub mime_type: String,
    pub content: Box<dyn Read + Send + Sync + 'static>,
}
