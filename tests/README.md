# Integration Tests

This directory contains integration tests for the kintone-rs library that test real API interactions with a Kintone environment.

## Prerequisites

To run these tests, you need:

1. Access to a Kintone environment (cybozu.com or on-premises)
2. A user account with the following permissions:
   - App creation permissions
   - App management permissions
   - Record management permissions

## Environment Variables

Set the following environment variables before running the tests:

```bash
export KINTONE_BASE_URL=https://your-domain.cybozu.com
export KINTONE_USERNAME=your-username
export KINTONE_PASSWORD=your-password
```

## Running the Tests

The integration tests are marked with `#[ignore]` attribute to prevent them from running during normal `cargo test` execution, since they require external dependencies and environment setup.

To run all integration tests:

```bash
cargo test --test integration_test -- --ignored
```

To run a specific integration test:

```bash
cargo test --test integration_test integration_test_full_workflow -- --ignored
```
