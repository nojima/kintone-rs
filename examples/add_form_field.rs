//! # Add Form Field Example
//!
//! This example demonstrates how to add fields to a Kintone app's form using the add_form_field API.
//! The example shows how to create various types of fields including text, number, date, and choice fields.
//!
//! ## Usage
//!
//! ```bash
//! cargo run --example add_form_field
//! ```
//!
//! ## Environment Variables
//!
//! Set the following environment variables before running:
//! - `KINTONE_BASE_URL`: Your Kintone domain (e.g., "https://example.cybozu.com")
//! - `KINTONE_USERNAME`: Your Kintone username
//! - `KINTONE_PASSWORD`: Your Kintone password
//! - `KINTONE_APP_ID`: The ID of the app to add fields to

use kintone::client::{Auth, KintoneClient};
use kintone::model::app::field::{
    Alignment, DateFieldProperty, FieldOption, FieldProperty, MultiLineTextFieldProperty,
    NumberFieldProperty, RadioButtonFieldProperty, SingleLineTextFieldProperty, UnitPosition,
};
use kintone::v1::app::form;
use std::collections::HashMap;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read configuration from environment variables
    let base_url =
        env::var("KINTONE_BASE_URL").expect("KINTONE_BASE_URL environment variable is required");
    let username =
        env::var("KINTONE_USERNAME").expect("KINTONE_USERNAME environment variable is required");
    let password =
        env::var("KINTONE_PASSWORD").expect("KINTONE_PASSWORD environment variable is required");
    let app_id: u64 = env::var("KINTONE_APP_ID")
        .expect("KINTONE_APP_ID environment variable is required")
        .parse()
        .expect("KINTONE_APP_ID must be a valid number");

    // Create client with username/password authentication
    // Note: App management APIs require username/password authentication, API tokens cannot be used
    let client = KintoneClient::new(&base_url, Auth::password(username, password));

    println!("Adding form fields to app {app_id}");

    // Create various field types to demonstrate the API

    // 1. Single Line Text Field
    let customer_name_field = FieldProperty::SingleLineText(SingleLineTextFieldProperty {
        code: "customer_name".to_string(),
        label: "Customer Name".to_string(),
        required: true,
        max_length: Some(100),
        ..Default::default()
    });

    // 2. Number Field
    let price_field = FieldProperty::Number(NumberFieldProperty {
        code: "price".to_string(),
        label: "Price".to_string(),
        required: true,
        min_value: Some(0.into()),
        digit: true,            // Show thousand separators
        display_scale: Some(2), // 2 decimal places
        unit: Some("USD".to_string()),
        unit_position: Some(UnitPosition::Before),
        ..Default::default()
    });

    // 3. Date Field
    let order_date_field = FieldProperty::Date(DateFieldProperty {
        code: "order_date".to_string(),
        label: "Order Date".to_string(),
        required: true,
        default_now_value: true, // Default to current date
        ..Default::default()
    });

    // 4. Radio Button Field (choice field)
    let mut priority_options = HashMap::new();
    priority_options.insert(
        "High".to_string(),
        FieldOption {
            label: "High".to_string(),
            index: 0,
        },
    );
    priority_options.insert(
        "Medium".to_string(),
        FieldOption {
            label: "Medium".to_string(),
            index: 1,
        },
    );
    priority_options.insert(
        "Low".to_string(),
        FieldOption {
            label: "Low".to_string(),
            index: 2,
        },
    );

    let priority_field = FieldProperty::RadioButton(RadioButtonFieldProperty {
        code: "priority".to_string(),
        label: "Priority".to_string(),
        required: true,
        options: priority_options,
        default_value: Some("Medium".to_string()),
        align: Some(Alignment::Horizontal),
        ..Default::default()
    });

    // 5. Multi-line Text Field
    let description_field = FieldProperty::MultiLineText(MultiLineTextFieldProperty {
        code: "description".to_string(),
        label: "Description".to_string(),
        ..Default::default()
    });

    // Send the request to add all fields
    let response = form::add_form_field(app_id)
        .field("customer_name", customer_name_field)
        .field("price", price_field)
        .field("order_date", order_date_field)
        .field("priority", priority_field)
        .field("description", description_field)
        .send(&client)?;

    println!("Successfully added form fields!");
    println!("New app revision: {}", response.revision);
    println!();
    println!("Note: The fields have been added to the preview environment.");
    println!("To apply these changes to the production environment, you need to deploy the app.");
    println!("You can use the deploy_app API or deploy manually from the app settings interface.");

    Ok(())
}
