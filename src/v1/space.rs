use crate::client::{KintoneClient, Request};
use crate::internal::serde_helper::as_str;
use crate::models::ThreadComment;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddThreadCommentRequest {
    space: u64,
    thread: u64,
    comment: ThreadComment,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddThreadCommentResponse {
    #[serde(with = "as_str")]
    pub id: u64,
}

impl AddThreadCommentRequest {
    pub fn call(self, client: &KintoneClient) -> crate::Result<AddThreadCommentResponse> {
        let req: Request<'_, AddThreadCommentRequest> =
            Request::builder(Method::POST, "/k/v1/space/thread/comment.json")
                .body(self)
                .build();
        Ok(client.call(req)?)
    }
}

#[must_use]
pub fn add_thread_comment(
    space: u64,
    thread: u64,
    comment: ThreadComment,
) -> AddThreadCommentRequest {
    AddThreadCommentRequest {
        space,
        thread,
        comment,
    }
}
