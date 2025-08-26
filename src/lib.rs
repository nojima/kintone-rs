//! A client library of Kintone REST APIs for Rust.
//!
//! **DISCLAIMER**: this OSS is my own personal work and does not have any relationship with Cybozu Inc. or any other organization which I belong to.
//!
//! **WARNING**: This library is under development and is likely to undergo incompatible changes in the future.
//!
//! ## Installation
//!
//! Add the following line to your `Cargo.toml` under the `[dependencies]` section:
//!
//! ```toml
//! kintone = { git = "https://github.com/nojima/kintone-rs" }
//! ```
//!
//! ## Usage
//!
//! This library provides a fluent API for interacting with Kintone REST APIs using method chaining. All API functions return request builders that can be configured with additional parameters and then sent to the Kintone server.
//!
//! ### Basic Example
//!
//! Here's a simple example that retrieves a record from a Kintone app and displays it:
//!
//! ```no_run
//! use kintone::client::{Auth, KintoneClient};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a client with your Kintone base URL and API token
//!     let base_url = "https://your-domain.cybozu.com";
//!     let api_token = "your-api-token".to_owned();
//!     let client = KintoneClient::new(base_url, Auth::api_token(api_token));
//!
//!     // Get a single record by ID
//!     let response = kintone::v1::record::get_record(123, 456)  // app_id: 123, record_id: 456
//!         .send(&client)?;
//!
//!     println!("Retrieved record:");
//!     for (field_code, field_value) in response.record.fields() {
//!         println!("  '{}' = {:?}", field_code, field_value);
//!     }
//!
//!     // Get multiple records with filtering
//!     let response = kintone::v1::record::get_records(123)
//!         .query("status = \"Active\"")
//!         .fields(&["name", "email", "status"])
//!         .send(&client)?;
//!
//!     for record in response.records {
//!         println!("Record:");
//!         for (field_code, field_value) in record.fields() {
//!             println!("  '{}' = {:?}", field_code, field_value);
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Supported APIs
//!
//! The library currently supports the following Kintone REST API endpoints:
//!
//! - [`v1::record`]: Record management APIs
//!     - [`v1::record::get_record`], [`v1::record::get_records`], [`v1::record::add_record`], [`v1::record::add_records`], [`v1::record::update_record`], [`v1::record::update_records`], [`v1::record::delete_records`], [`v1::record::bulk_request`], [`v1::record::update_assignees`], [`v1::record::update_status`], [`v1::record::get_comments`], [`v1::record::add_comment`], [`v1::record::delete_comment`], [`v1::record::create_cursor`], [`v1::record::get_records_by_cursor`], [`v1::record::delete_cursor`]
//! - [`v1::file`]: File management APIs
//!     - [`v1::file::upload`], [`v1::file::download`]
//! - [`v1::space`]: Space management APIs
//!     - [`v1::space::add_space`], [`v1::space::delete_space`], [`v1::space::add_thread`], [`v1::space::add_thread_comment`]
//! - [`v1::app`]: App management APIs
//!     - [`v1::app::add_app`], [`v1::app::settings::deploy_app`], [`v1::app::settings::get_app_deploy_status`], [`v1::app::form::add_form_field`]
//!
//! ### Builder Pattern and Method Chaining
//!
//! Each API function follows the same pattern: create a request builder, optionally configure it
//! with additional parameters using method chaining, and then call `.send(&client)` to execute the request.
//!
//! ```no_run
//! # use std::error::Error;
//! # use kintone::client::{Auth, KintoneClient};
//! # let client = KintoneClient::new("https://example.cybozu.com", Auth::api_token("token".to_owned()));
//! # let app_id = 1;
//! let response = kintone::v1::record::get_records(app_id) // Returns a request builder
//!     .query("status = \"Active\"") // Optional parameter: query filter
//!     .fields(&["name", "email"])   // Optional parameter: field selection
//!     .send(&client)?;              // Execute the request
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! This builder pattern allows us to add optional parameters to our APIs while maintaining
//! backward compatibility. You can call multiple methods on the builder to configure your
//! request, and only the final `.send(&client)` call actually sends the request to the server.
//!
//! **Note**: If you forget to call `.send(&client)`, the compiler will help you catch this mistake
//! with a helpful warning:
//!
//! ```text
//! unused `kintone::v1::space::AddThreadCommentRequest` that must be used
//! ```
//!
//! This happens because all request builders are marked with `#[must_use]`, so you'll
//! quickly notice if you accidentally create a request without sending it. The compiler
//! warning ensures you won't miss this important step!
//!
//! ## Examples
//!
//! For more detailed examples and usage patterns, check out the `examples` directory in this repository.
//! Each example demonstrates how to use specific API endpoints and can be run directly to test the functionality.
//!
//! The examples use environment variables for configuration:
//!
//! - `KINTONE_BASE_URL`: Your Kintone domain URL (e.g., `https://your-domain.cybozu.com`)
//! - `KINTONE_API_TOKEN`: Your API token for authentication
//! - `KINTONE_USERNAME`: Your username for Kintone
//! - `KINTONE_PASSWORD`: Your password for Kintone
//!
//! You can run an example like this:
//!
//! ```bash
//! export KINTONE_BASE_URL=https://your-domain.cybozu.com
//! export KINTONE_API_TOKEN=your-token
//! export KINTONE_USERNAME=your-username
//! export KINTONE_PASSWORD=your-password
//! cargo run --example get_record
//! ```

pub mod client;
pub mod error;
pub mod middleware;
pub mod model;
pub mod v1;

mod internal;
