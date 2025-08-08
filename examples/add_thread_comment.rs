use std::error::Error;

use kintone::client::{Auth, KintoneClient};
use kintone::models::space::ThreadComment;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let base_url = std::env::var("KINTONE_BASE_URL").expect("KINTONE_BASE_URL is not set");
    let username = std::env::var("KINTONE_USERNAME").expect("KINTONE_USERNAME is not set");
    let password = std::env::var("KINTONE_PASSWORD").expect("KINTONE_PASSWORD is not set");

    let client = KintoneClient::new(&base_url, Auth::password(username, password));

    let comment = ThreadComment {
        text: "Hello, World!".to_owned(),
        mentions: vec![],
    };
    let resp = kintone::v1::space::add_thread_comment(2, 4, comment).send(&client)?;

    println!("resp = {resp:?}");
    Ok(())
}
