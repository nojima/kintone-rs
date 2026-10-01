//! # Kintone REST API v1
//!
//! This module provides access to Kintone's REST API version 1 endpoints.
//! It contains submodules for different categories of API operations.
//!
//! ## Available API Categories
//!
//! - [`app`] - App management operations (create apps in preview environment)
//! - [`record`] - Record management operations (CRUD operations, comments, workflow)
//! - [`mod@file`] - File upload and download operations
//! - [`space`] - Space and thread management operations
//! - [`plugin`] - Plugin management
//! - [`apis`] - API discovery and schemas

pub mod apis;
pub mod app;
pub mod file;
pub mod plugin;
pub mod record;
pub mod space;

#[cfg(test)]
mod tests;
