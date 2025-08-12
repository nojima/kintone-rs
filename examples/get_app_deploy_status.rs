//! # Get App Deploy Status Example
//!
//! This example demonstrates how to check the deployment status of Kintone apps
//! using the get_app_deploy_status API. This is useful for monitoring the progress
//! of app deployments initiated with the deploy_app API.
//!
//! ## Usage
//!
//! ```bash
//! cargo run --example get_app_deploy_status
//! ```
//!
//! ## Environment Variables
//!
//! Set the following environment variables before running:
//! - `KINTONE_BASE_URL`: Your Kintone domain (e.g., "https://example.cybozu.com")
//! - `KINTONE_USERNAME`: Your Kintone username
//! - `KINTONE_PASSWORD`: Your Kintone password
//! - `KINTONE_APP_IDS`: Comma-separated list of app IDs to check (e.g., "123,124,125")
//!
//! Note: This example requires username/password authentication as app management APIs
//! cannot use API tokens.

use kintone::client::{Auth, KintoneClient};
use kintone::v1::app::settings;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read configuration from environment variables
    let base_url =
        env::var("KINTONE_BASE_URL").expect("KINTONE_BASE_URL environment variable is required");
    let username =
        env::var("KINTONE_USERNAME").expect("KINTONE_USERNAME environment variable is required");
    let password =
        env::var("KINTONE_PASSWORD").expect("KINTONE_PASSWORD environment variable is required");
    let app_ids_str =
        env::var("KINTONE_APP_IDS").expect("KINTONE_APP_IDS environment variable is required");

    // Parse app IDs from comma-separated string
    let app_ids: Result<Vec<u64>, _> =
        app_ids_str.split(',').map(|s| s.trim().parse::<u64>()).collect();

    let app_ids = app_ids.map_err(|e| format!("Failed to parse app IDs: {e}"))?;

    if app_ids.is_empty() {
        return Err("No app IDs provided".into());
    }

    // Create client with username/password authentication
    let client = KintoneClient::new(&base_url, Auth::password(username, password));

    println!("🔍 Checking deployment status for {} app(s)...\n", app_ids.len());

    // Build the request to check multiple apps
    let mut request = settings::get_app_deploy_status();
    for &app_id in &app_ids {
        request = request.app(app_id);
    }

    // Send the request
    let response = request.send(&client)?;

    println!("📊 Deployment Status Results:");
    println!("{}", "─".repeat(50));

    for app_status in &response.apps {
        let status_icon = match app_status.status {
            settings::DeployStatus::Processing => "⏳",
            settings::DeployStatus::Success => "✅",
            settings::DeployStatus::Fail => "❌",
            settings::DeployStatus::Cancel => "⚠️",
        };

        let status_text = match app_status.status {
            settings::DeployStatus::Processing => "PROCESSING - Deployment in progress",
            settings::DeployStatus::Success => "SUCCESS - Deployment completed successfully",
            settings::DeployStatus::Fail => "FAIL - Deployment failed",
            settings::DeployStatus::Cancel => "CANCEL - Deployment was cancelled",
        };

        println!("{} App ID {}: {}", status_icon, app_status.app, status_text);
    }

    println!("{}", "─".repeat(50));

    // Summary
    let processing_count = response
        .apps
        .iter()
        .filter(|a| a.status == settings::DeployStatus::Processing)
        .count();
    let success_count = response
        .apps
        .iter()
        .filter(|a| a.status == settings::DeployStatus::Success)
        .count();
    let fail_count = response
        .apps
        .iter()
        .filter(|a| a.status == settings::DeployStatus::Fail)
        .count();
    let cancel_count = response
        .apps
        .iter()
        .filter(|a| a.status == settings::DeployStatus::Cancel)
        .count();

    println!("\n📈 Summary:");
    println!("   • Total apps checked: {}", response.apps.len());
    if processing_count > 0 {
        println!("   • Processing: {processing_count}");
    }
    if success_count > 0 {
        println!("   • Successful: {success_count}");
    }
    if fail_count > 0 {
        println!("   • Failed: {fail_count}");
    }
    if cancel_count > 0 {
        println!("   • Cancelled: {cancel_count}");
    }

    if processing_count > 0 {
        println!(
            "\n💡 Tip: Apps still processing can be checked again later using this same command."
        );
    }

    if fail_count > 0 || cancel_count > 0 {
        println!(
            "\n⚠️  Some deployments were not successful. Please check the Kintone interface for more details."
        );
    }

    Ok(())
}
