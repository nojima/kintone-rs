//! Example demonstrating client certificate authentication for Kintone Secure Access
//!
//! This example shows how to use client certificates to authenticate with Kintone
//! when the "Secure Access" feature is enabled. Secure Access requires mutual TLS (mTLS)
//! authentication using client certificates.
//!
//! ## Prerequisites
//!
//! 1. Kintone environment with Secure Access enabled
//! 2. Client certificate in PFX format downloaded from cybozu.com
//! 3. OpenSSL installed for certificate conversion
//!
//! ## Certificate Conversion
//!
//! First, convert the downloaded PFX certificate to PEM format:
//!
//! ```bash
//! openssl pkcs12 -in your-cert.pfx -nokeys -out client-cert.pem
//! openssl pkcs12 -in your-cert.pfx -nocerts -out client-key.pem -nodes
//! ```
//!
//! ## Environment Variables
//!
//! Set the following environment variables:
//! ```bash
//! # Do not forget ".s"
//! export KINTONE_BASE_URL=https://your-domain.s.cybozu.com
//! export KINTONE_API_TOKEN=your-api-token
//! export KINTONE_APP_ID=123
//! export CLIENT_CERT_PATH=./client-cert.pem
//! export CLIENT_KEY_PATH=./client-key.pem
//! ```
//!
//! ## Running the Example
//!
//! ```bash
//! cargo run --example client_certificate
//! ```

use std::env;

use kintone::{
    client::{Auth, KintoneClientBuilder},
    v1::record,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from environment variables
    let base_url =
        env::var("KINTONE_BASE_URL").expect("Please set KINTONE_BASE_URL environment variable");
    let api_token =
        env::var("KINTONE_API_TOKEN").expect("Please set KINTONE_API_TOKEN environment variable");
    let app_id: u64 = env::var("KINTONE_APP_ID")
        .unwrap_or_else(|_| "1".to_string())
        .parse()
        .expect("KINTONE_APP_ID must be a valid number");

    let cert_path =
        env::var("CLIENT_CERT_PATH").expect("Please set CLIENT_CERT_PATH environment variable");
    let key_path =
        env::var("CLIENT_KEY_PATH").expect("Please set CLIENT_KEY_PATH environment variable");

    println!("Setting up Kintone client with client certificate authentication...");
    println!("Certificate: {cert_path}");
    println!("Private key: {key_path}");
    println!();

    // Load certificate and private key from files
    println!("Loading certificate and private key...");
    let cert_pem = std::fs::read(&cert_path)?;
    let key_pem = std::fs::read(&key_path)?;

    // Create client with client certificate
    let client = KintoneClientBuilder::new(&base_url, Auth::api_token(api_token))
        .client_certificate_from_pem(&cert_pem, &key_pem)?
        .build();

    // Get records
    match record::get_records(app_id).send(&client) {
        Ok(response) => {
            println!("Successfully retrieved {} records", response.records.len());

            println!("Display first few records:");
            for (i, record) in response.records.iter().take(3).enumerate() {
                println!("   [{i}]: {record:?}");
            }
        }
        Err(e) => {
            eprintln!("Failed to get records: {e}");
            return Err(e.into());
        }
    }
    println!();

    Ok(())
}
