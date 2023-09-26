use crate::client::KintoneClient;
use crate::models::ThreadComment;
use crate::{client::RequestBuilder, internal::serde_helper::stringified};
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[must_use]
pub struct AddThreadCommentRequest {
    builder: RequestBuilder,
    body: AddThreadCommentRequestBody,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddThreadCommentRequestBody {
    space: u64,
    thread: u64,
    comment: ThreadComment,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddThreadCommentResponse {
    #[serde(with = "stringified")]
    pub id: u64,
}

impl AddThreadCommentRequest {
    pub fn send(self) -> crate::Result<AddThreadCommentResponse> {
        Ok(self.builder.body(&self.body).send()?)
    }
}

#[must_use]
pub fn add_thread_comment(
    client: &KintoneClient,
    space: u64,
    thread: u64,
    comment: ThreadComment,
) -> AddThreadCommentRequest {
    AddThreadCommentRequest {
        builder: client.request(Method::POST, "/k/v1/space/thread/comment.json"),
        body: AddThreadCommentRequestBody {
            space,
            thread,
            comment,
        },
    }
}
