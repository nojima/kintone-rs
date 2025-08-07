use std::io::Read;
use serde::Deserialize;

use crate::ApiResult;
use crate::client::{KintoneClient, UploadRequest};

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
