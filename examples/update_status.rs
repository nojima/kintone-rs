use std::error::Error;

use kintone::client::{Auth, KintoneClient};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let base_url = std::env::var("KINTONE_BASE_URL").expect("KINTONE_BASE_URL is not set");
    let username = std::env::var("KINTONE_USERNAME").expect("KINTONE_USERNAME is not set");
    let password = std::env::var("KINTONE_PASSWORD").expect("KINTONE_PASSWORD is not set");

    let client = KintoneClient::new(&base_url, Auth::password(username, password));

    // 基本的なステータス更新（アクションのみ）
    let resp = kintone::v1::record::update_status(1, 1, "申請する".to_owned()).send(&client)?;
    println!("Status updated successfully. New revision: {}", resp.revision);

    // 作業者とリビジョンを指定したステータス更新
    let resp = kintone::v1::record::update_status(1, 2, "承認する".to_owned())
        .assignee("user2".to_owned())
        .revision(5)
        .send(&client)?;
    println!("Status updated with assignee. New revision: {}", resp.revision);

    Ok(())
}
