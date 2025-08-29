# kintone-rs

[![Crates.io](https://img.shields.io/crates/v/kintone.svg)](https://crates.io/crates/kintone)
[![Documentation](https://docs.rs/kintone/badge.svg)](https://docs.rs/kintone)

**DISCLAIMER**: this OSS is my own personal work and does not have any relationship with Cybozu Inc. or any other organization which I belong to.

**WARNING**: This library is under development and is likely to undergo incompatible changes in the future.

A client library of Kintone REST APIs for Rust.

## Quick Start

```rust
use kintone::client::{Auth, KintoneClient};

// Create a client
let client = KintoneClient::new(
    "https://your-domain.cybozu.com",
    Auth::api_token("your-api-token")
);

// Get a record
let response = kintone::v1::record::get_record(app_id, record_id)
    .send(&client)?;

// Print the record
for (field_code, field_value) in response.record.fields() {
    println!("{}: {:?}", field_code, field_value);
}
```

For detailed documentation, installation instructions, and usage examples, please refer to the [API documentation](https://docs.rs/kintone).

## Middleware Support

kintone-rs supports a middleware system for handling cross-cutting concerns like retries, logging, and authentication. Middleware layers can be easily composed to add functionality to your Kintone client.

### Available Middleware

- **RetryLayer**: Automatically retries failed requests with exponential backoff
- **LoggingLayer**: Logs HTTP request and response information for debugging
- **BasicAuthLayer**: Adds HTTP Basic authentication headers

### Example: Retry

```rust
use std::time::Duration;
use kintone::client::{Auth, KintoneClient};
use kintone::middleware;

let client = KintoneClient::builder(
        "https://your-domain.cybozu.com",
        Auth::api_token("your-api-token")
    )
    .layer(middleware::RetryLayer::new())
    .build();
```

## Examples

You can find runnable examples in the `examples` directory.

The examples require environment variables to be set:

```bash
export KINTONE_BASE_URL=https://your-domain.cybozu.com
export KINTONE_API_TOKEN=your-api-token
cargo run --example get_record
```
