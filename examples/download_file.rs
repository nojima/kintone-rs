use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};

use kintone::client::{Auth, KintoneClient};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let base_url = std::env::var("KINTONE_BASE_URL").expect("KINTONE_BASE_URL is not set");
    let username = std::env::var("KINTONE_USERNAME").expect("KINTONE_USERNAME is not set");
    let password = std::env::var("KINTONE_PASSWORD").expect("KINTONE_PASSWORD is not set");

    let client = KintoneClient::new(&base_url, Auth::password(username, password));

    // ファイルをダウンロードして保存
    let file_key = std::env::var("FILE_KEY")
        .unwrap_or_else(|_| "201202061155587E339F9067544F1A92C743460E3D12B3297".to_string());

    let resp = kintone::v1::file::download(file_key).send(&client)?;

    println!("Downloaded file with MIME type: {}", resp.mime_type);

    // ファイルに保存
    let output_file = File::create("downloaded_file.bin")?;
    let mut writer = BufWriter::new(output_file);

    let mut buffer = [0; 8192];
    let mut total_bytes = 0;
    let mut content = resp.content;

    loop {
        let bytes_read = content.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        writer.write_all(&buffer[..bytes_read])?;
        total_bytes += bytes_read;
    }

    writer.flush()?;
    println!("Downloaded {total_bytes} bytes to downloaded_file.bin");

    Ok(())
}
