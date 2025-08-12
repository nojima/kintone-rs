//! # Integration Tests for kintone-rs
//!
//! This module contains integration tests that test the complete workflow
//! of creating apps, adding fields, deploying apps, and managing records
//! using the real Kintone API.
//!
//! ## Setup
//!
//! To run these tests, you need to set the following environment variables:
//!
//! - `KINTONE_BASE_URL`: Your Kintone domain URL (e.g., `https://your-domain.cybozu.com`)
//! - `KINTONE_USERNAME`: Your username for Kintone
//! - `KINTONE_PASSWORD`: Your password for Kintone
//!
//! ## Running the Tests
//!
//! These tests are marked with `#[ignore]` because they require a real Kintone environment.
//! To run them, use:
//!
//! ```bash
//! export KINTONE_BASE_URL=https://your-domain.cybozu.com
//! export KINTONE_USERNAME=your-username
//! export KINTONE_PASSWORD=your-password
//! cargo test --test integration_test -- --ignored
//! ```
//!
//! ## Test Scenarios
//!
//! - `integration_test_full_workflow`: Complete app creation, field addition, deployment, record management, and querying
//! - `integration_test_record_operations`: Record CRUD operations (Create, Read, Update)

use std::{
    env,
    thread::{self, sleep},
    time::Duration,
};

use bigdecimal::BigDecimal;
use kintone::{
    client::{Auth, KintoneClient, KintoneClientBuilder},
    middleware,
    model::{
        app::field::{FieldProperty, NumberFieldProperty, SingleLineTextFieldProperty},
        record::{FieldValue, Record},
    },
    v1::{app, record},
};

// Test configuration structure
struct TestConfig {
    base_url: String,
    username: String,
    password: String,
}

impl TestConfig {
    fn from_env() -> Result<Self, String> {
        let base_url = env::var("KINTONE_BASE_URL")
            .map_err(|_| "KINTONE_BASE_URL environment variable is required")?;
        let username = env::var("KINTONE_USERNAME")
            .map_err(|_| "KINTONE_USERNAME environment variable is required")?;
        let password = env::var("KINTONE_PASSWORD")
            .map_err(|_| "KINTONE_PASSWORD environment variable is required")?;

        Ok(TestConfig {
            base_url,
            username,
            password,
        })
    }

    fn create_client(&self) -> KintoneClient {
        KintoneClientBuilder::new(
            &self.base_url,
            Auth::password(self.username.clone(), self.password.clone()),
        )
        .layer(middleware::LoggingLayer::new())
        .layer(middleware::RetryLayer::new(5, Duration::from_secs(1), Duration::from_secs(8), None))
        .build()
    }
}

#[test]
#[ignore] // This test requires real Kintone environment setup
fn integration_test_full_workflow() {
    let config =
        TestConfig::from_env().expect("Failed to load test configuration from environment");
    let client = config.create_client();

    // 1. Create an app
    let app_name = format!("Test App {}", chrono::Utc::now().timestamp());
    let create_response = app::add_app(&app_name).send(&client).expect("Failed to create app");

    let app_id = create_response.app;
    println!("Created app with ID: {app_id}");

    sleep(Duration::from_secs(2));

    // 2. Add fields to the app
    let text_field = SingleLineTextFieldProperty {
        code: "name".to_owned(),
        label: "Name".to_owned(),
        required: true,
        max_length: Some(100),
        ..Default::default()
    };

    let number_field = NumberFieldProperty {
        code: "age".to_owned(),
        label: "Age".to_owned(),
        required: false,
        min_value: Some(BigDecimal::from(0)),
        max_value: Some(BigDecimal::from(150)),
        ..Default::default()
    };

    let add_field_response = app::form::add_form_field(app_id)
        .field(FieldProperty::SingleLineText(text_field))
        .field(FieldProperty::Number(number_field))
        .send(&client)
        .expect("Failed to add fields");

    println!("Added fields, new revision: {}", add_field_response.revision);

    // 3. Deploy the app and wait for completion
    app::settings::deploy_app()
        .app(app_id, Some(add_field_response.revision))
        .send(&client)
        .expect("Failed to start deployment");

    println!("Deployment started, waiting for completion...");

    // Wait for deployment to complete
    let max_attempts = 30; // Maximum 30 attempts (30 seconds)
    let mut attempts = 0;

    loop {
        attempts += 1;
        if attempts > max_attempts {
            panic!("Deployment did not complete within {max_attempts} seconds");
        }

        let status_response = app::settings::get_app_deploy_status()
            .app(app_id)
            .send(&client)
            .expect("Failed to check deployment status");

        if let Some(app_status) = status_response.apps.first() {
            match app_status.status {
                app::settings::DeployStatus::Success => {
                    println!("Deployment completed successfully");
                    break;
                }
                app::settings::DeployStatus::Fail => {
                    panic!("Deployment failed");
                }
                app::settings::DeployStatus::Cancel => {
                    panic!("Deployment was cancelled");
                }
                app::settings::DeployStatus::Processing => {
                    println!("Deployment still in progress (attempt {attempts}/{max_attempts})");
                    thread::sleep(Duration::from_secs(1));
                    continue;
                }
            }
        } else {
            panic!("No deployment status found for app {app_id}");
        }
    }

    // 4. Add some records to the app
    let test_records = vec![("Alice", 25), ("Bob", 30), ("Charlie", 35), ("Diana", 28)];

    let mut record_ids = Vec::new();

    for (name, age) in &test_records {
        let mut record = Record::new();
        record.put_field("name", FieldValue::SingleLineText(name.to_string()));
        record.put_field("age", FieldValue::Number(BigDecimal::from(*age)));

        let add_record_response = record::add_record(app_id)
            .record(record)
            .send(&client)
            .expect("Failed to add record");

        record_ids.push(add_record_response.id);
        println!("Added record for {} with ID: {}", name, add_record_response.id);
    }

    // 5. Retrieve records by ID and verify they match expectations
    for (i, &record_id) in record_ids.iter().enumerate() {
        let get_response = record::get_record(app_id, record_id)
            .send(&client)
            .expect("Failed to get record");

        let retrieved_record = &get_response.record;
        let expected_name = test_records[i].0;
        let expected_age = test_records[i].1;

        // Verify name field
        if let Some(FieldValue::SingleLineText(name)) = retrieved_record.get("name") {
            assert_eq!(name, expected_name, "Name field mismatch for record {record_id}");
        } else {
            panic!("Name field not found or wrong type for record {record_id}");
        }

        // Verify age field
        if let Some(FieldValue::Number(age_decimal)) = retrieved_record.get("age") {
            let age: i32 = age_decimal.to_string().parse().expect("Failed to parse age");
            assert_eq!(age, expected_age, "Age field mismatch for record {record_id}");
        } else {
            panic!("Age field not found or wrong type for record {record_id}");
        }

        println!("✓ Record {record_id} verified: {expected_name} (age {expected_age})");
    }

    // 6. Retrieve records with filter conditions and verify results
    // Test filter: age >= 30
    let filter_response = record::get_records(app_id)
        .query("age >= 30")
        .fields(&["name", "age"])
        .send(&client)
        .expect("Failed to get records with filter");

    let filtered_records = &filter_response.records;

    // We expect Bob (30), Charlie (35) to match the filter
    assert_eq!(filtered_records.len(), 2, "Expected 2 records with age >= 30");

    let mut found_names = Vec::new();
    for record in filtered_records {
        if let Some(FieldValue::SingleLineText(name)) = record.get("name") {
            found_names.push(name.clone());
        }
    }

    found_names.sort();
    let mut expected_names = vec!["Bob".to_string(), "Charlie".to_string()];
    expected_names.sort();

    assert_eq!(found_names, expected_names, "Filtered records don't match expectations");
    println!("✓ Filter test passed: Found {} records with age >= 30", filtered_records.len());

    println!("🎉 All integration tests passed!");
}

#[test]
#[ignore] // This test requires real Kintone environment setup
fn integration_test_record_operations() {
    let config =
        TestConfig::from_env().expect("Failed to load test configuration from environment");
    let client = config.create_client();

    // Create a simple app for record operations
    let app_name = format!("Record Test App {}", chrono::Utc::now().timestamp());
    let create_response = app::add_app(&app_name).send(&client).expect("Failed to create app");

    let app_id = create_response.app;

    sleep(Duration::from_secs(2));

    // Add a simple text field
    let text_field = SingleLineTextFieldProperty {
        code: "title".to_owned(),
        label: "Title".to_owned(),
        required: true,
        max_length: Some(200),
        ..Default::default()
    };

    let add_field_response = app::form::add_form_field(app_id)
        .field(FieldProperty::SingleLineText(text_field))
        .send(&client)
        .expect("Failed to add field");

    // Deploy the app
    app::settings::deploy_app()
        .app(app_id, Some(add_field_response.revision))
        .send(&client)
        .expect("Failed to start deployment");

    // Wait for deployment
    let max_attempts = 20;
    for attempt in 1..=max_attempts {
        let status_response = app::settings::get_app_deploy_status()
            .app(app_id)
            .send(&client)
            .expect("Failed to check deployment status");

        if let Some(app_status) = status_response.apps.first() {
            match app_status.status {
                app::settings::DeployStatus::Success => break,
                app::settings::DeployStatus::Fail | app::settings::DeployStatus::Cancel => {
                    panic!("Deployment failed or was cancelled");
                }
                app::settings::DeployStatus::Processing => {
                    if attempt == max_attempts {
                        panic!("Deployment timeout");
                    }
                    thread::sleep(Duration::from_secs(1));
                }
            }
        }
    }

    // Test record CRUD operations
    // Create record
    let mut record = Record::new();
    record.put_field("title", FieldValue::SingleLineText("Test Record".to_owned()));

    let add_response = record::add_record(app_id)
        .record(record)
        .send(&client)
        .expect("Failed to add record");

    let record_id = add_response.id;
    println!("Created record with ID: {record_id}");

    // Read record
    let get_response = record::get_record(app_id, record_id)
        .send(&client)
        .expect("Failed to get record");

    if let Some(FieldValue::SingleLineText(title)) = get_response.record.get("title") {
        assert_eq!(title, "Test Record");
        println!("✓ Record read test passed");
    } else {
        panic!("Title field not found or wrong type");
    }

    // Update record
    let mut update_record = Record::new();
    update_record.put_field("title", FieldValue::SingleLineText("Updated Test Record".to_owned()));

    let update_response = record::update_record(app_id)
        .id(record_id)
        .record(update_record)
        .revision(get_response.record.revision().unwrap())
        .send(&client)
        .expect("Failed to update record");

    println!("Updated record to revision: {}", update_response.revision);

    // Verify update
    let get_updated_response = record::get_record(app_id, record_id)
        .send(&client)
        .expect("Failed to get updated record");

    if let Some(FieldValue::SingleLineText(title)) = get_updated_response.record.get("title") {
        assert_eq!(title, "Updated Test Record");
        println!("✓ Record update test passed");
    } else {
        panic!("Updated title field not found or wrong type");
    }

    println!("🎉 Record operations test passed!");
}
