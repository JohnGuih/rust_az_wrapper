//! Utilities for executing Azure CLI commands

use crate::error::{AzureError, Result};
use serde::de::DeserializeOwned;
use std::process::{Command, Stdio};

/// Executes an Azure CLI command and returns the result as parsed JSON
pub async fn execute_az_command<T>(args: &[&str]) -> Result<T>
where
    T: DeserializeOwned,
{
    let output = execute_az_command_raw(args).await?;
    
    if output.is_empty() {
        return Err(AzureError::CliExecution {
            command: format!("az {}", args.join(" ")),
            error: "Empty output".to_string(),
        });
    }

    let parsed: T = serde_json::from_str(&output)?;
    Ok(parsed)
}

/// Executes an Azure CLI command and returns the raw output as string
pub async fn execute_az_command_raw(args: &[&str]) -> Result<String> {
    let mut command = Command::new("az");
    command
        .args(args)
        .arg("--output")
        .arg("json")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let output = command.output()?;
    
    if !output.status.success() {
        return Err(AzureError::from_command_output(
            &format!("az {}", args.join(" ")),
            output,
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.to_string())
}

/// Checks if Azure CLI is installed and accessible
pub async fn check_az_cli() -> Result<()> {
    let mut command = Command::new("az");
    command
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    match command.output() {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                Err(AzureError::CliNotFound)
            }
        }
        Err(_) => Err(AzureError::CliNotFound),
    }
}

/// Checks if the user is authenticated with Azure CLI
pub async fn check_authentication() -> Result<()> {
    let result = execute_az_command_raw(&["account", "show"]).await;
    
    match result {
        Ok(_) => Ok(()),
        Err(AzureError::CliError { stderr, .. }) if stderr.contains("az login") => {
            Err(AzureError::Authentication)
        }
        Err(e) => Err(e),
    }
}

/// Formats optional parameters for Azure CLI commands
pub fn format_optional_param(param: &str, value: &Option<String>) -> Vec<String> {
    match value {
        Some(v) => vec![param.to_string(), v.clone()],
        None => vec![],
    }
}

/// Formats boolean parameters for Azure CLI commands
pub fn format_bool_param(param: &str, value: Option<bool>) -> Vec<String> {
    match value {
        Some(true) => vec![param.to_string()],
        Some(false) => vec![format!("{}=false", param)],
        None => vec![],
    }
}

/// Builds arguments for Azure CLI commands
pub struct AzCommandBuilder {
    args: Vec<String>,
}

impl AzCommandBuilder {
    /// Creates a new builder
    pub fn new() -> Self {
        Self { args: Vec::new() }
    }

    /// Adds a subcommand
    pub fn subcommand(mut self, cmd: &str) -> Self {
        self.args.push(cmd.to_string());
        self
    }

    /// Adds a required parameter
    pub fn param(mut self, param: &str, value: &str) -> Self {
        self.args.push(param.to_string());
        self.args.push(value.to_string());
        self
    }

    /// Adds an optional parameter
    pub fn optional_param(mut self, param: &str, value: &Option<String>) -> Self {
        if let Some(v) = value {
            self.args.push(param.to_string());
            self.args.push(v.clone());
        }
        self
    }

    /// Adds a boolean flag
    pub fn flag(mut self, flag: &str) -> Self {
        self.args.push(flag.to_string());
        self
    }

    /// Adds a conditional flag
    pub fn conditional_flag(mut self, flag: &str, condition: bool) -> Self {
        if condition {
            self.args.push(flag.to_string());
        }
        self
    }

    /// Adds the subscription flag if provided
    pub fn subscription(mut self, subscription: Option<&str>) -> Self {
        if let Some(sub) = subscription {
            self.args.push("--subscription".to_string());
            self.args.push(sub.to_string());
        }
        self
    }

    /// Builds the final arguments
    pub fn build(self) -> Vec<String> {
        self.args
    }

    /// Executes the command and returns the parsed result
    pub async fn execute<T>(self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let args: Vec<&str> = self.args.iter().map(|s| s.as_str()).collect();
        execute_az_command(&args).await
    }

    /// Executes the command and returns the raw result
    pub async fn execute_raw(self) -> Result<String> {
        let args: Vec<&str> = self.args.iter().map(|s| s.as_str()).collect();
        execute_az_command_raw(&args).await
    }
}

impl Default for AzCommandBuilder {
    fn default() -> Self {
        Self::new()
    }
} 