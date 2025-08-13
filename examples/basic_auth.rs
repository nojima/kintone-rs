//! # Basic Authentication Example
//!
//! This example demonstrates how to use the BasicAuthLayer middleware to add
//! HTTP Basic authentication to Kintone API requests. This is useful when
//! accessing Kintone through proxy servers or reverse proxies that require
//! HTTP Basic authentication.
//!
//! ## Usage
//!
//! Set the following environment variables:
//! - `KINTONE_BASE_URL`: Your Kintone base URL (e.g., "https://your-domain.cybozu.com")
//! - `KINTONE_API_TOKEN`: Your Kintone API token
//! - `KINTONE_APP_ID`: The app ID to access
//! - `BASIC_AUTH_USERNAME`: Basic authentication username
//! - `BASIC_AUTH_PASSWORD`: Basic authentication password
//!
//! ```bash
//! export KINTONE_BASE_URL="https://your-domain.cybozu.com"
//! export KINTONE_API_TOKEN="your-api-token"
//! export KINTONE_APP_ID="123"
//! export BASIC_AUTH_USERNAME="proxy_user"
//! export BASIC_AUTH_PASSWORD="proxy_password"
//! cargo run --example basic_auth
//! ```

use std::env;

use kintone::client::{Auth, KintoneClientBuilder};
use kintone::middleware;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get configuration from environment variables
    let base_url =
        env::var("KINTONE_BASE_URL").expect("KINTONE_BASE_URL environment variable is required");
    let api_token =
        env::var("KINTONE_API_TOKEN").expect("KINTONE_API_TOKEN environment variable is required");
    let app_id: u64 = env::var("KINTONE_APP_ID")
        .expect("KINTONE_APP_ID environment variable is required")
        .parse()
        .expect("KINTONE_APP_ID must be a valid number");

    // Basic authentication credentials
    let basic_username = env::var("BASIC_AUTH_USERNAME")
        .expect("BASIC_AUTH_USERNAME environment variable is required");
    let basic_password = env::var("BASIC_AUTH_PASSWORD")
        .expect("BASIC_AUTH_PASSWORD environment variable is required");

    println!("Connecting to Kintone with Basic authentication...");
    println!("Base URL: {base_url}");
    println!("App ID: {app_id}");
    println!("Basic Auth User: {basic_username}");

    // Create Kintone client with Basic authentication layer
    let client = KintoneClientBuilder::new(&base_url, Auth::api_token(api_token))
        .layer(middleware::BasicAuthLayer::new(&basic_username, &basic_password))
        .build();

    // Test the connection by fetching records
    println!("\nFetching records from app {app_id}...");

    let response = kintone::v1::record::get_records(app_id).send(&client)?;

    println!("✅ Successfully fetched {} records!", response.records.len());

    for (i, record) in response.records.iter().enumerate() {
        let id = record.id().map(|id| id.to_string()).unwrap_or_else(|| "N/A".to_string());
        println!("  [{}]: ID = {}", i + 1, id);
    }

    Ok(())
}
