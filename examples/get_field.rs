//! Example: Get specific fields from a Kintone record
//!
//! This example demonstrates how to use both `v1::app::get_apps` and `v1::record::get_record`
//! to fetch specific fields from a record identified by app code and numerical ID.
//!
//! Usage: cargo run --example get_field -- RECORD_ID FIELD...
//!
//! Where:
//! - RECORD_ID: App code and numerical ID connected by hyphen (e.g., TASK-123)
//! - FIELD...: Field codes to retrieve (e.g., $id summary description)
//!
//! Output: JSON object containing the requested fields
//! Example output: { "$id": "1", "summary": "...", "description": "..." }

use std::env;
use std::error::Error;

use kintone::{
    client::{Auth, KintoneClient},
    model::record::FieldValue,
};
use serde_json::{Map, Value};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: cargo run --example get_field -- RECORD_ID FIELD...");
        eprintln!(
            "  RECORD_ID: App code and numerical ID connected by hyphen (e.g., TASK-123, FOOBAR-42)"
        );
        eprintln!("  FIELD...: Field codes to retrieve (e.g., $id summary description)");
        std::process::exit(1);
    }

    let record_id = &args[1];
    let fields: Vec<&str> = args[2..].iter().map(|s| s.as_str()).collect();

    // Parse RECORD_ID into app code and numerical ID
    let (app_code, numerical_id) = parse_record_id(record_id)?;

    let base_url = env::var("KINTONE_BASE_URL").expect("KINTONE_BASE_URL is not set");
    let username = env::var("KINTONE_USERNAME").expect("KINTONE_USERNAME is not set");
    let password = env::var("KINTONE_PASSWORD").expect("KINTONE_PASSWORD is not set");

    let client = KintoneClient::new(&base_url, Auth::password(username, password));

    // Find app by code
    let app_id = find_app_by_code(&client, &app_code)?;

    // Get the record (note: single record API doesn't support field filtering)
    let resp = kintone::v1::record::get_record(app_id, numerical_id).send(&client)?;

    // Convert selected fields to JSON
    let mut result = Map::new();
    for field_code in &fields {
        if let Some(field_value) = resp.record.get(field_code) {
            let value = extract_inner_field_value(field_value).expect("Failed to get field value");
            result.insert(field_code.to_string(), value);
        }
    }

    let json_output = serde_json::to_string_pretty(&Value::Object(result))?;
    println!("{json_output}");

    Ok(())
}

fn parse_record_id(record_id: &str) -> Result<(String, u64), Box<dyn Error + Send + Sync>> {
    let parts: Vec<&str> = record_id.split('-').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid RECORD_ID format: '{record_id}'. Expected format: APP_CODE-NUMERICAL_ID (e.g., TASK-123)").into());
    }

    let app_code = parts[0].to_string();
    let numerical_id = parts[1]
        .parse::<u64>()
        .map_err(|_| format!("Invalid numerical ID: '{}'. Must be a positive integer", parts[1]))?;

    Ok((app_code, numerical_id))
}

fn find_app_by_code(
    client: &KintoneClient,
    app_code: &str,
) -> Result<u64, Box<dyn Error + Send + Sync>> {
    let resp = kintone::v1::app::get_apps().codes([app_code]).send(client)?;

    if resp.apps.is_empty() {
        return Err(format!("App with code '{app_code}' not found").into());
    }

    if resp.apps.len() > 1 {
        return Err(
            format!("Multiple apps found with code '{app_code}'. This should not happen.").into()
        );
    }

    Ok(resp.apps[0].app_id)
}

fn extract_inner_field_value(
    field_value: &FieldValue,
) -> Result<serde_json::Value, Box<dyn Error>> {
    let outer = serde_json::to_value(field_value)?;
    let object = outer.as_object().ok_or("Failed to convert field value to object")?;
    let inner = object.get("value").ok_or("Failed to get 'value' from field value object")?;
    Ok(inner.clone())
}
