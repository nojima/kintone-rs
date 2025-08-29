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
    Alignment, FieldOption, UnitPosition, date_field_property, multi_line_text_field_property,
    number_field_property, radio_button_field_property, single_line_text_field_property,
};
use kintone::v1::app::form;
use std::collections::BTreeMap;
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
    let customer_name_field = single_line_text_field_property("customer_name")
        .label("Customer Name")
        .required(true)
        .max_length(100)
        .build();

    // 2. Number Field
    let price_field = number_field_property("price")
        .label("Price")
        .required(true)
        .min_value(0.into())
        .digit(true) // Show thousand separators
        .display_scale(2) // 2 decimal places
        .unit("USD")
        .unit_position(UnitPosition::Before)
        .build();

    // 3. Date Field
    let order_date_field = date_field_property("order_date")
        .label("Order Date")
        .required(true)
        .default_now_value(true) // Default to current date
        .build();

    // 4. Radio Button Field (choice field)
    let mut priority_options = BTreeMap::new();
    priority_options.insert(
        "High".to_owned(),
        FieldOption {
            label: "High".to_owned(),
            index: 0,
        },
    );
    priority_options.insert(
        "Medium".to_owned(),
        FieldOption {
            label: "Medium".to_owned(),
            index: 1,
        },
    );
    priority_options.insert(
        "Low".to_owned(),
        FieldOption {
            label: "Low".to_owned(),
            index: 2,
        },
    );

    let priority_field = radio_button_field_property("priority")
        .label("Priority")
        .required(true)
        .options(priority_options)
        .default_value("Medium")
        .align(Alignment::Horizontal)
        .build();

    // 5. Multi-line Text Field
    let description_field =
        multi_line_text_field_property("description").label("Description").build();

    // Send the request to add all fields
    let response = form::add_form_field(app_id)
        .field(customer_name_field.into())
        .field(price_field.into())
        .field(order_date_field.into())
        .field(priority_field.into())
        .field(description_field.into())
        .send(&client)?;

    println!("Successfully added form fields!");
    println!("New app revision: {}", response.revision);
    println!();
    println!("Note: The fields have been added to the preview environment.");
    println!("To apply these changes to the production environment, you need to deploy the app.");
    println!("You can use the deploy_app API or deploy manually from the app settings interface.");

    Ok(())
}
