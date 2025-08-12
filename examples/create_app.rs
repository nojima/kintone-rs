//! # Create App with Field and Deploy Example
//!
//! This example demonstrates the complete workflow of:
//! 1. Creating a new app in the preview environment
//! 2. Adding a field to the app's form
//! 3. Deploying the app to the production environment
//!
//! This shows how to use the kintone-rs library for the full app creation workflow.
//!
//! ## Usage
//!
//! ```bash
//! cargo run --example create_app
//! ```
//!
//! ## Environment Variables
//!
//! Set the following environment variables before running:
//! - `KINTONE_BASE_URL`: Your Kintone domain (e.g., "https://example.cybozu.com")
//! - `KINTONE_USERNAME`: Your Kintone username
//! - `KINTONE_PASSWORD`: Your Kintone password
//!
//! Note: This example requires username/password authentication as app management APIs
//! cannot use API tokens.

use kintone::client::{Auth, KintoneClient};
use kintone::model::app::field::SingleLineTextFieldProperty;
use kintone::v1::app::{self, form, settings};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read configuration from environment variables
    let base_url =
        env::var("KINTONE_BASE_URL").expect("KINTONE_BASE_URL environment variable is required");
    let username =
        env::var("KINTONE_USERNAME").expect("KINTONE_USERNAME environment variable is required");
    let password =
        env::var("KINTONE_PASSWORD").expect("KINTONE_PASSWORD environment variable is required");

    // Create client with username/password authentication
    // Note: App management APIs require username/password authentication, API tokens cannot be used
    let client = KintoneClient::new(&base_url, Auth::password(username, password));

    println!("🚀 Starting app creation workflow...\n");

    // Step 1: Create a new app in the preview environment
    println!("📱 Step 1: Creating new app...");
    let app_response = app::add_app("Customer Management App").send(&client)?;

    let app_id = app_response.app;
    let initial_revision = app_response.revision;

    println!("   ✅ App created successfully!");
    println!("   📋 App ID: {app_id}");
    println!("   🔢 Initial revision: {initial_revision}\n");

    // Step 2: Add multiple fields to the app's form
    println!("🏗️  Step 2: Adding fields to app form...");

    // Create a customer name field
    let customer_name_field = SingleLineTextFieldProperty {
        code: "customer_name".to_owned(),
        label: "Customer Name".to_owned(),
        required: true,
        max_length: Some(50),
        min_length: Some(1),
        ..Default::default()
    };

    // Create an email field
    let email_field = SingleLineTextFieldProperty {
        code: "email".to_owned(),
        label: "Email Address".to_owned(),
        required: true,
        unique: true, // Email should be unique
        ..Default::default()
    };

    // Create a phone number field
    let phone_field = SingleLineTextFieldProperty {
        code: "phone".to_owned(),
        label: "Phone Number".to_owned(),
        max_length: Some(20),
        ..Default::default()
    };

    // Add all fields at once
    let field_response = form::add_form_field(app_id)
        .field("customer_name", customer_name_field.into())
        .field("email", email_field.into())
        .field("phone", phone_field.into())
        .revision(Some(initial_revision))
        .send(&client)?;

    let field_revision = field_response.revision;

    println!("   ✅ Fields added successfully!");
    println!("   📝 Added fields:");
    println!("     • customer_name: Customer Name (required, max 100 chars)");
    println!("     • email: Email Address (required, unique, max 255 chars)");
    println!("     • phone: Phone Number (optional, max 20 chars)");
    println!("   🔢 New revision: {field_revision}\n");

    // Step 3: Deploy the app to production environment
    println!("🚀 Step 3: Deploying app to production...");

    settings::deploy_app().app(app_id, Some(field_revision)).send(&client)?;

    println!("   ✅ App deployment initiated!");

    // Step 4: Check deployment status
    println!("⏳ Step 4: Checking deployment status...");

    // Poll deployment status until completion
    let mut attempts = 0;
    let max_attempts = 30; // Wait up to 30 seconds

    loop {
        attempts += 1;

        let status_response = settings::get_app_deploy_status().app(app_id).send(&client)?;

        if let Some(app_status) = status_response.apps.first() {
            match app_status.status {
                settings::DeployStatus::Processing => {
                    println!("   ⏳ Deployment in progress... (attempt {attempts}/{max_attempts})");
                    if attempts >= max_attempts {
                        println!(
                            "   ⚠️  Deployment is taking longer than expected. Please check manually."
                        );
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
                settings::DeployStatus::Success => {
                    println!("   ✅ Deployment completed successfully!");
                    break;
                }
                settings::DeployStatus::Fail => {
                    println!("   ❌ Deployment failed!");
                    return Err("App deployment failed".into());
                }
                settings::DeployStatus::Cancel => {
                    println!("   ⚠️  Deployment was cancelled!");
                    return Err("App deployment was cancelled".into());
                }
            }
        } else {
            println!("   ❌ No status information received for app");
            break;
        }
    }

    println!();

    // Summary
    println!("🎉 Workflow completed successfully!");
    println!("📊 Summary:");
    println!("   • App ID: {app_id}");
    println!("   • App Name: Customer Management App");
    println!("   • Added Fields:");
    println!("     - customer_name: Customer Name (required)");
    println!("     - email: Email Address (required, unique)");
    println!("     - phone: Phone Number (optional)");
    println!("   • Final Revision: {field_revision}");
    println!("   • Status: Deployed to production");
    println!();
    println!("💡 Next steps:");
    println!("   • Access your app at: {base_url}/k/{app_id}/");
    println!("   • Add more fields using form::add_form_field");
    println!("   • Create records using the record APIs");
    println!("   • Set up permissions and workflows as needed");

    Ok(())
}
