//! # Kintone REST API v1
//!
//! This module provides access to Kintone's REST API version 1 endpoints.
//! It contains submodules for different categories of API operations.
//!
//! ## Available API Categories
//!
//! - [`record`] - Record management operations (CRUD operations, comments, workflow)
//! - [`file`] - File upload and download operations
//! - [`space`] - Space and thread management operations

pub mod file;
pub mod record;
pub mod space;
