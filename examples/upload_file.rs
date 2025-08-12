use std::error::Error;
use std::fs::File;

use kintone::client::{Auth, KintoneClient};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let base_url = std::env::var("KINTONE_BASE_URL").expect("KINTONE_BASE_URL is not set");
    let username = std::env::var("KINTONE_USERNAME").expect("KINTONE_USERNAME is not set");
    let password = std::env::var("KINTONE_PASSWORD").expect("KINTONE_PASSWORD is not set");

    let client = KintoneClient::new(&base_url, Auth::password(username, password));

    // ファイルからアップロード
    let file_path = "sample.txt";
    let file = File::open(file_path)?;

    let resp = kintone::v1::file::upload("sample.txt".to_owned()).send(&client, file)?;
    println!("File uploaded successfully. File key: {}", resp.file_key);

    // バイト配列からアップロード
    let content = b"Hello, World! This is a test file.";

    let resp = kintone::v1::file::upload("test.txt".to_owned()).send(&client, &content[..])?;
    println!("Content uploaded successfully. File key: {}", resp.file_key);

    Ok(())
}
