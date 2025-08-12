use std::error::Error;

use kintone::client::{Auth, KintoneClient};
use kintone::model::{Entity, EntityType, record::RecordComment};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let base_url = std::env::var("KINTONE_BASE_URL").expect("KINTONE_BASE_URL is not set");
    let username = std::env::var("KINTONE_USERNAME").expect("KINTONE_USERNAME is not set");
    let password = std::env::var("KINTONE_PASSWORD").expect("KINTONE_PASSWORD is not set");

    let client = KintoneClient::new(&base_url, Auth::password(username, password));

    // 基本的なコメント投稿
    let resp = kintone::v1::record::add_comment(5, 1, RecordComment::from_text("Hello, World!"))
        .send(&client)?;

    println!("Basic comment added with ID: {}", resp.id);

    // メンションを含むコメント投稿
    let mentions = vec![
        Entity {
            type_: EntityType::USER,
            code: "takahashi".to_owned(),
        },
        Entity {
            type_: EntityType::GROUP,
            code: "sample_group".to_owned(),
        },
    ];

    let comment = RecordComment {
        text: "Please review this record.".to_owned(),
        mentions,
    };

    let resp = kintone::v1::record::add_comment(5, 1, comment).send(&client)?;

    println!("Comment with mentions added with ID: {}", resp.id);

    Ok(())
}
