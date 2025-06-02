//! # Rust Azure CLI Wrapper for Cosmos DB
//!
//! A type-safe and ergonomic Rust wrapper for Azure CLI operations focused on **Cosmos DB**.
//! 
//! This wrapper abstracts Azure CLI calls related to Cosmos DB and provides an ergonomic Rust API for:
//! 
//! - **Subscriptions**: List and manage Azure subscriptions
//! - **Cosmos DB Accounts**: Create, list, update and delete accounts  
//! - **Databases**: Manage SQL and MongoDB databases
//! - **Containers**: Manage containers and collections
//! - **Keys and Connection Strings**: Get access keys and connection strings
//! - **Throughput**: Manage throughput settings
//! - **Failover**: Configure failover between regions
//! 
//! All results are returned as Rust objects that can be easily converted to JSON.
//! 
//! ## Quick Example
//! 
//! ```rust,no_run
//! use rust_az_wrapper::AzureClient;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = AzureClient::new()?;
//!     
//!     // List Cosmos DB accounts
//!     let accounts = client.list_cosmos_accounts(None).await?;
//!     println!("Found {} Cosmos accounts", accounts.len());
//!     
//!     // Convert to JSON
//!     let json = serde_json::to_string_pretty(&accounts)?;
//!     println!("{}", json);
//!     
//!     Ok(())
//! }
//! ```

pub use crate::client::AzureClient;
pub use crate::error::{AzureError, Result};
pub use crate::models::*;

pub mod client;
pub mod commands;
pub mod error;
pub mod models;
pub mod utils; 