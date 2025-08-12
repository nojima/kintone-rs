use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

use kintone::client::{Auth, KintoneClient};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let base_url = std::env::var("KINTONE_BASE_URL").expect("KINTONE_BASE_URL is not set");
    let username = std::env::var("KINTONE_USERNAME").expect("KINTONE_USERNAME is not set");
    let password = std::env::var("KINTONE_PASSWORD").expect("KINTONE_PASSWORD is not set");

    let client = KintoneClient::new(&base_url, Auth::password(username, password));

    // 1. ファイルをアップロード
    let file_path = "sample.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let upload_resp = kintone::v1::file::upload("sample.txt".to_owned()).send(&client, reader)?;
    println!(
        "File uploaded successfully. File key: {}",
        upload_resp.file_key
    );

    // 2. アップロードしたファイルをダウンロード
    let download_resp = kintone::v1::file::download(upload_resp.file_key).send(&client)?;
    println!(
        "Downloaded file with MIME type: {}",
        download_resp.mime_type
    );

    // 3. ダウンロードしたファイルを保存
    let output_file = File::create("downloaded_sample.txt")?;
    let mut writer = BufWriter::new(output_file);

    let mut buffer = [0; 8192];
    let mut total_bytes = 0;
    let mut content = download_resp.content;

    loop {
        let bytes_read = content.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        writer.write_all(&buffer[..bytes_read])?;
        total_bytes += bytes_read;
    }

    writer.flush()?;
    println!("Downloaded {total_bytes} bytes to downloaded_sample.txt");

    // 4. ファイル内容を確認
    let downloaded_content = std::fs::read_to_string("downloaded_sample.txt")?;
    println!("Downloaded content:");
    println!("{downloaded_content}");

    Ok(())
}
