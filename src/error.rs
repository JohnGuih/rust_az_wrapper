//! Error handling for Azure CLI operations

use std::process::Output;
use thiserror::Error;

/// Result type for Azure operations
pub type Result<T> = std::result::Result<T, AzureError>;

/// Errors that can occur when using the Azure CLI wrapper
#[derive(Debug, Error)]
pub enum AzureError {
    /// Azure CLI is not installed or not accessible
    #[error("Azure CLI is not installed or not accessible. Please install Azure CLI and ensure it's in your PATH.")]
    CliNotFound,

    /// User is not authenticated with Azure CLI
    #[error("Not authenticated with Azure. Please run 'az login' first.")]
    Authentication,

    /// Error executing Azure CLI command
    #[error("Failed to execute Azure CLI command '{command}': {error}")]
    CliExecution { command: String, error: String },

    /// Azure CLI returned an error
    #[error("Azure CLI error in command '{command}': {stderr}")]
    CliError { command: String, stderr: String },

    /// Failed to parse JSON response from Azure CLI
    #[error("Failed to parse JSON response: {0}")]
    JsonParse(#[from] serde_json::Error),

    /// IO error (e.g., failed to spawn process)
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Generic error with custom message
    #[error("{0}")]
    Custom(String),
}

impl AzureError {
    /// Creates an AzureError from command output
    pub fn from_command_output(command: &str, output: Output) -> Self {
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        if !stderr.is_empty() {
            AzureError::CliError {
                command: command.to_string(),
                stderr: stderr.to_string(),
            }
        } else {
            AzureError::CliExecution {
                command: command.to_string(),
                error: format!("Command failed with exit code: {}", 
                    output.status.code().unwrap_or(-1)),
            }
        }
    }
} 