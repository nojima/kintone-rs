use std::error::Error;

use kintone::client::{Auth, KintoneClient};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let base_url = std::env::var("KINTONE_BASE_URL").expect("KINTONE_BASE_URL is not set");
    let api_token = std::env::var("KINTONE_API_TOKEN").expect("KINTONE_API_TOKEN is not set");

    let client = KintoneClient::new(&base_url, Auth::api_token(api_token));

    // 基本的な作業者更新
    let resp =
        kintone::v1::record::update_assignees(1, 1, vec!["user1".to_owned(), "user2".to_owned()])
            .send(&client)?;

    println!("Assignees updated, new revision: {}", resp.revision);

    // リビジョン番号を指定して作業者更新
    let resp = kintone::v1::record::update_assignees(1, 2, vec!["user3".to_owned()])
        .revision(5)
        .send(&client)?;

    println!("Assignees updated with revision check, new revision: {}", resp.revision);

    // 作業者を解除（空の配列を指定）
    let resp = kintone::v1::record::update_assignees(1, 3, vec![]).send(&client)?;

    println!("Assignees cleared, new revision: {}", resp.revision);

    Ok(())
}
