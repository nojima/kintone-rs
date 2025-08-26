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
//! **Note**: The space operations test requires space creation and deletion permissions.
//! This may require administrator privileges in your Kintone environment.
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
//! - `integration_test_space_operations`: Space and thread management operations

use std::{
    env,
    thread::{self, sleep},
    time::Duration,
};

use kintone::{
    client::{Auth, KintoneClient, KintoneClientBuilder},
    middleware,
    model::{
        app::field::{FieldProperty, NumberFieldProperty, SingleLineTextFieldProperty},
        record::{FieldValue, Record},
    },
    v1::{app, record},
};

fn setup_logger() {
    // https://docs.rs/env_logger/latest/env_logger/#specifying-defaults-for-environment-variables
    // https://docs.rs/env_logger/latest/env_logger/#capturing-logs-in-tests
    let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .is_test(true)
        .try_init();
}

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
        .layer(middleware::RetryLayer::new())
        .layer(middleware::LoggingLayer::new())
        .build()
    }
}

/// Waits for app deployment to complete, polling the status at regular intervals.
///
/// # Arguments
/// * `client` - The Kintone client to use for API calls
/// * `app_id` - The ID of the app being deployed
/// * `max_attempts` - Maximum number of polling attempts before timeout
///
/// # Panics
/// Panics if deployment fails, is cancelled, times out, or no status is found.
fn wait_for_deployment_completion(client: &KintoneClient, app_id: u64, max_attempts: u32) {
    println!("Deployment started, waiting for completion...");

    for attempt in 1..=max_attempts {
        let status_response = app::settings::get_app_deploy_status()
            .app(app_id)
            .send(client)
            .expect("Failed to check deployment status");

        if let Some(app_status) = status_response.apps.first() {
            match app_status.status {
                app::settings::DeployStatus::Success => {
                    println!("Deployment completed successfully");
                    return;
                }
                app::settings::DeployStatus::Fail => {
                    panic!("Deployment failed");
                }
                app::settings::DeployStatus::Cancel => {
                    panic!("Deployment was cancelled");
                }
                app::settings::DeployStatus::Processing => {
                    if attempt == max_attempts {
                        panic!("Deployment did not complete within {max_attempts} attempts");
                    }
                    println!("Deployment still in progress (attempt {attempt}/{max_attempts})");
                    thread::sleep(Duration::from_secs(1));
                }
            }
        } else {
            panic!("No deployment status found for app {app_id}");
        }
    }
}

#[test]
#[ignore] // This test requires real Kintone environment setup
fn integration_test_full_workflow() {
    setup_logger();

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
        max_length: Some(50),
        ..Default::default()
    };

    let number_field = NumberFieldProperty {
        code: "age".to_owned(),
        label: "Age".to_owned(),
        required: false,
        min_value: Some(0.into()),
        max_value: Some(200.into()),
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

    wait_for_deployment_completion(&client, app_id, 30);

    // 4. Add some records to the app
    let test_records = vec![("Alice", 25), ("Bob", 30), ("Charlie", 35), ("Diana", 28)];

    let mut record_ids = Vec::new();

    for (name, age) in &test_records {
        let record = Record::from([
            ("name", FieldValue::SingleLineText(name.to_string())),
            ("age", FieldValue::Number(Some(age.into()))),
        ]);

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
        if let Some(FieldValue::Number(Some(age_decimal))) = retrieved_record.get("age") {
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

    let mut found_names: Vec<_> = filtered_records
        .iter()
        .filter_map(|record| match record.get("name") {
            Some(FieldValue::SingleLineText(name)) => Some(name.clone()),
            _ => None,
        })
        .collect();

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
    setup_logger();

    let config =
        TestConfig::from_env().expect("Failed to load test configuration from environment");
    let client = config.create_client();

    // Create a simple app for record operations
    let app_name = format!("Record Test App {}", chrono::Utc::now().timestamp());
    let create_response = app::add_app(&app_name).send(&client).expect("Failed to create app");

    let app_id = create_response.app;

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

    wait_for_deployment_completion(&client, app_id, 20);

    // Test record CRUD operations
    // Create record
    let record = Record::from([("title", FieldValue::SingleLineText("Test Record".to_owned()))]);

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
    let update_record =
        Record::from([("title", FieldValue::SingleLineText("Updated Test Record".to_owned()))]);

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

/*
#[test]
#[ignore] // This test requires real Kintone environment setup
fn integration_test_space_operations() {
    setup_logger();

    let config =
        TestConfig::from_env().expect("Failed to load test configuration from environment");
    let client = config.create_client();

    // Test space lifecycle: create -> add thread -> add comment -> delete
    let space_name = format!("Test Space {}", chrono::Utc::now().timestamp());

    // 1. Create a new space
    let create_space_response =
        space::add_space(&space_name).send(&client).expect("Failed to create space");

    let space_id = create_space_response.id;
    println!("Created space '{space_name}' with ID: {space_id}");

    // 2. Create a thread in the space
    let thread_name = "Integration Test Thread";
    let create_thread_response = space::add_thread(space_id, thread_name)
        .send(&client)
        .expect("Failed to create thread");

    let thread_id = create_thread_response.id;
    println!("Created thread '{thread_name}' with ID: {thread_id}");

    // 3. Add comments to the thread
    let comments = [
        "This is the first comment in our integration test.",
        "This is a second comment to test multiple comments.",
        "Final comment to complete the test scenario.",
    ];

    let mut comment_ids = Vec::new();

    for (i, comment_text) in comments.iter().enumerate() {
        let comment = ThreadComment {
            text: comment_text.to_string(),
            mentions: vec![], // No mentions in this basic test
        };

        let add_comment_response = space::add_thread_comment(space_id, thread_id, comment)
            .send(&client)
            .unwrap_or_else(|_| panic!("Failed to add comment {}", i + 1));

        comment_ids.push(add_comment_response.id);
        println!("Added comment {} with ID: {}", i + 1, add_comment_response.id);
    }

    // 4. Test comment with mentions
    let mention_comment = ThreadComment {
        text: "This comment mentions a user @user".to_string(),
        mentions: vec![Entity {
            entity_type: EntityType::USER,
            code: "user".to_string(),
        }],
    };

    let mention_response = space::add_thread_comment(space_id, thread_id, mention_comment)
        .send(&client)
        .expect("Failed to add comment with mentions");

    println!("Added comment with mentions, ID: {}", mention_response.id);

    // 5. Clean up: Delete the space
    // Note: This will delete the space and all its content (threads, comments)
    space::delete_space(space_id).send(&client).expect("Failed to delete space");

    println!("Successfully deleted space {space_id}");
}
*/
