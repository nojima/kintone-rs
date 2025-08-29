use std::fs::File;
use std::{error::Error, io::BufReader};

use kintone::{
    client::{Auth, KintoneClient},
    model::{
        file_body,
        record::{FieldValue, Record},
    },
};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let base_url = std::env::var("KINTONE_BASE_URL").expect("KINTONE_BASE_URL is not set");
    let username = std::env::var("KINTONE_USERNAME").expect("KINTONE_USERNAME is not set");
    let password = std::env::var("KINTONE_PASSWORD").expect("KINTONE_PASSWORD is not set");

    let client = KintoneClient::new(&base_url, Auth::password(username, password));

    // 1. ファイルをアップロード
    let file_path = "sample.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let upload_resp = kintone::v1::file::upload("日本語.txt".to_owned()).send(&client, reader)?;
    println!("File uploaded successfully. File key: {}", upload_resp.file_key);

    // 2. ファイルをレコードに添付（ビルダーパターンを使用）
    let app_id = 96;
    let resp = kintone::v1::record::add_record(app_id)
        .record(Record::from([(
            "Attachment",
            FieldValue::File(vec![file_body(upload_resp.file_key).build()]),
        )]))
        .send(&client)?;

    // 3. レコードを取得
    let resp = kintone::v1::record::get_record(app_id, resp.id).send(&client)?;
    let Some(FieldValue::File(files)) = resp.record.get("Attachment") else {
        panic!("Attachment field was not found");
    };
    if files.is_empty() {
        panic!("Attachment is empty");
    }
    println!("Attachment file: {:?}", files[0]);

    // 4. 添付ファイルをダウンロード
    let file_key_for_download = files[0].file_key.clone();
    let mut download_resp = kintone::v1::file::download(file_key_for_download).send(&client)?;
    println!("Downloaded file with MIME type: {:?}", download_resp.mime_type);

    // 5. ダウンロードしたファイルを保存
    let downloaded_file_path = "downloaded_sample.txt";
    let mut output_file = File::create(downloaded_file_path)?;
    let n_bytes = std::io::copy(&mut download_resp.content, &mut output_file)?;

    print!("Downloaded {n_bytes} bytes to {downloaded_file_path}");

    Ok(())
}
