# kintone-rs

DISCLAIMER: this OSS is my own personal work and does not have any relationship with Cybozu Inc. or any other organization which I belong to.

WARNING: This library is under development and is likely to undergo incompatible changes in the future.

A client library of Kintone REST APIs for Rust.

See examples in the `examples` directory.

## Install

Add following line in your `Cargo.toml` right below `[dependencies]`.

```toml
kintone = { git = "https://github.com/nojima/kintone-rs" }
```

## Usage

This library provides a fluent API for interacting with Kintone REST APIs using method chaining. All API functions return request builders that can be configured with additional parameters and then sent to the Kintone server.

### Basic Example

Here's a simple example that retrieves a record from a Kintone app and displays it:

```rust
use std::error::Error;
use kintone::client::{Auth, KintoneClient};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Create a client with your Kintone base URL and API token
    let base_url = "https://your-domain.cybozu.com";
    let api_token = "your-api-token";
    let client = KintoneClient::new(base_url, Auth::api_token(api_token));

    // Get a single record by ID
    let response = kintone::v1::record::get_record(123, 456)  // app_id: 123, record_id: 456
        .send(&client)?;
    
    println!("Retrieved record:");
    for (field_code, field_value) in response.record.fields() {
        println!("  '{}' = {:?}", field_code, field_value);
    }
    
    // Get multiple records with filtering
    let response = kintone::v1::record::get_records(123)
        .query("status = \"Active\"")
        .fields(&["name", "email", "status"])
        .send(&client)?;
    
    for record in response.records {
        println!("Record:");
        for (field_code, field_value) in record.fields() {
            println!("  '{}' = {:?}", field_code, field_value);
        }
    }

    Ok(())
}
```

### APIs

The library currently supports the following Kintone REST APIs:

- `kintone::v1::record`:
    - `get_record`, `get_records`, `add_record`, `update_record`, `update_assignees`, `update_status`, `get_comments`, `add_comment`, `delete_comment`
- `kintone::v1::file`:
    - `upload`, `download`
- `kintone::v1::thread`:
    - `add_thread_comment`

Each function follows the same pattern: create a request builder, optionally configure it with additional parameters, and then call `.send(&client)` to execute the request.

```rust
let response = kintone::v1::record::get_records(app_id) // `get_records` returns a request builder
    .query("status = \"Active\"") // `query` is optional argument
    .fields(&["name", "email"])   // `fields` is also optional argument
    .send(&client)                // send the request to the server
```

With this pattern, we can add optional arguments to the our API without breaking backward compatibility.

For more detailed examples and usage patterns, check out the `examples` directory in this repository. Each example demonstrates how to use specific API endpoints and can be run directly to test the functionality.

**Note**: If you forget to call `.send(&client)`, don't worry! The compiler will remind you with a helpful warning like:

```
unused `kintone::v1::space::AddThreadCommentRequest` that must be used
```

This happens because request builders are marked with `#[must_use]`, so you'll quickly notice if you accidentally create a request without sending it.

## More Examples

You can find examples in the `./examples` directory.
These examples takes some parameters from environment variables:

- `KINTONE_BASE_URL`: Your Kintone domain URL (e.g., `https://your-domain.cybozu.com`)
- `KINTONE_API_TOKEN`: Your API token for authentication
- `KINTONE_USERNAME`: Your user name for Kintone
- `KINTONE_PASSWORD`: Your password for Kintone

You can run examples like this:

```bash
export KINTONE_BASE_URL=https://your-domain.cybozu.com
export KINTONE_API_TOKEN=your-token
export KINTONE_USERNAME=your-username
export KINTONE_PASSWORD=your-password
cargo run --example get_record
```
