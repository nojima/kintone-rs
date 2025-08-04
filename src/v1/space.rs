use serde::{Deserialize, Serialize};

use crate::ApiResult;
use crate::client::{KintoneClient, RequestBuilder};
use crate::internal::serde_helper::stringified;
use crate::models::ThreadComment;

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
    pub fn send(self) -> ApiResult<AddThreadCommentResponse> {
        self.builder.send(self.body)
    }
}

pub fn add_thread_comment(
    client: &KintoneClient,
    space: u64,
    thread: u64,
    comment: ThreadComment,
) -> AddThreadCommentRequest {
    AddThreadCommentRequest {
        builder: client.request(http::Method::POST, "/k/v1/space/thread/comment.json"),
        body: AddThreadCommentRequestBody {
            space,
            thread,
            comment,
        },
    }
}
