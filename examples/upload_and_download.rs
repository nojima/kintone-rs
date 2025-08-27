use std::error::Error;
use std::fs::File;

use kintone::{client::{Auth, KintoneClient, KintoneClientBuilder}, middleware::LoggingLayer};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let base_url = std::env::var("KINTONE_BASE_URL").expect("KINTONE_BASE_URL is not set");
    let username = std::env::var("KINTONE_USERNAME").expect("KINTONE_USERNAME is not set");
    let password = std::env::var("KINTONE_PASSWORD").expect("KINTONE_PASSWORD is not set");

    env_logger::init();
    let client = KintoneClientBuilder::new(&base_url, Auth::password(username, password))
        .layer(LoggingLayer::new())
        .build();

    // 1. ファイルをアップロード
    let file_path = "sample.txt";
    let file = File::open(file_path)?;

    let upload_resp = kintone::v1::file::upload("sample.txt".to_owned()).send(&client, file)?;
    println!("File uploaded successfully. File key: {}", upload_resp.file_key);

    // 2. アップロードしたファイルをダウンロード
    let mut download_resp = kintone::v1::file::download(upload_resp.file_key).send(&client)?;
    println!("Downloaded file with MIME type: {:?}", download_resp.mime_type);

    // 3. ダウンロードしたファイルを保存
    let mut output_file = File::create("downloaded_sample.txt")?;
    let n_bytes = std::io::copy(&mut download_resp.content, &mut output_file)?;

    print!("Downloaded {n_bytes} bytes to downloaded_sample.txt");

    Ok(())
}
