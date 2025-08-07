use std::error::Error;

use kintone::client::{Auth, KintoneClient};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let base_url = std::env::var("KINTONE_BASE_URL").expect("KINTONE_BASE_URL is not set");
    let username = std::env::var("KINTONE_USERNAME").expect("KINTONE_USERNAME is not set");
    let password = std::env::var("KINTONE_PASSWORD").expect("KINTONE_PASSWORD is not set");

    let client = KintoneClient::new(&base_url, Auth::password(username, password));

    // コメントを削除
    let resp = kintone::v1::record::delete_comment(1, 1, 1).send(&client)?;

    println!("Comment deleted successfully: {resp:?}");

    Ok(())
}
