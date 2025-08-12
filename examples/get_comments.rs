use std::error::Error;

use kintone::{
    client::{Auth, KintoneClient},
    model::Order,
};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let base_url = std::env::var("KINTONE_BASE_URL").expect("KINTONE_BASE_URL is not set");
    let username = std::env::var("KINTONE_USERNAME").expect("KINTONE_USERNAME is not set");
    let password = std::env::var("KINTONE_PASSWORD").expect("KINTONE_PASSWORD is not set");

    let client = KintoneClient::new(&base_url, Auth::password(username, password));

    let resp = kintone::v1::record::get_comments(1, 1)
        .order(Order::Desc)
        .offset(0)
        .limit(10)
        .send(&client)?;

    println!("resp = {resp:?}");
    Ok(())
}
