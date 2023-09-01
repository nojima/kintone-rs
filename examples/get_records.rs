use kintone::client::{Auth, KintoneClient};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let base_url = std::env::var("KINTONE_BASE_URL").expect("KINTONE_BASE_URL is not set");
    let api_token = std::env::var("KINTONE_API_TOKEN").expect("KINTONE_API_TOKEN is not set");

    let client = KintoneClient::new(&base_url, Auth::api_token(api_token))?;
    let resp = kintone::v1::record::get_records(5).call(&client)?;

    for record in resp.records {
        println!("{{");
        for (field_code, field_value) in record.fields() {
            println!("    '{field_code}' = {field_value:?}");
        }
        println!("}}");
    }
    Ok(())
}
