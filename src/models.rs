//! Data models for Azure resources focused on Cosmos DB

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Azure Subscription information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    /// Subscription ID
    pub id: String,
    /// Subscription name
    #[serde(alias = "name")]
    pub display_name: String,
    /// Subscription state
    pub state: String,
    /// Tenant ID
    #[serde(alias = "tenantId")]
    pub tenant_id: String,
    /// Whether this is the default subscription
    #[serde(alias = "isDefault")]
    pub is_default: Option<bool>,
}

/// Resource Group information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceGroup {
    /// Resource group name
    pub name: String,
    /// Resource group ID
    pub id: String,
    /// Resource group location
    pub location: String,
    /// Associated tags
    pub tags: Option<HashMap<String, String>>,
    /// Provisioning state within properties
    pub properties: Option<ResourceGroupProperties>,
}

/// Resource Group properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceGroupProperties {
    /// Provisioning state
    #[serde(alias = "provisioningState", alias = "provisioning_state")]
    pub provisioning_state: String,
}

/// Cosmos DB Account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmosAccount {
    /// Account name
    pub name: String,
    /// Resource ID
    pub id: String,
    /// Account location
    pub location: String,
    /// Resource group
    #[serde(alias = "resourceGroup", alias = "resource_group")]
    pub resource_group: String,
    /// Resource type
    #[serde(rename = "type", alias = "resourceType", alias = "resource_type")]
    pub resource_type: String,
    /// Account type (SQL, MongoDB, etc.)
    pub kind: String,
    /// Provisioning status
    #[serde(alias = "provisioningState", alias = "provisioning_state")]
    pub provisioning_state: String,
    /// Endpoint URI
    #[serde(alias = "documentEndpoint", alias = "document_endpoint")]
    pub document_endpoint: String,
    /// Associated tags
    pub tags: Option<HashMap<String, String>>,
    
    // Cosmos DB specific fields directly at root level
    /// Backup policy
    #[serde(alias = "backupPolicy")]
    pub backup_policy: Option<BackupPolicy>,
    /// Analytical storage configuration
    #[serde(alias = "analyticalStorageConfiguration")]
    pub analytical_storage_configuration: Option<AnalyticalStorageConfiguration>,
    /// API properties
    #[serde(alias = "apiProperties")]
    pub api_properties: Option<ApiProperties>,
    /// Account capabilities
    pub capabilities: Option<Vec<Capability>>,
    /// Capacity configuration
    pub capacity: Option<CapacitySettings>,
    /// Consistency policy
    #[serde(alias = "consistencyPolicy")]
    pub consistency_policy: Option<ConsistencyPolicy>,
    /// CORS settings
    pub cors: Option<Vec<CorsPolicy>>,
    /// Creation mode
    #[serde(alias = "createMode")]
    pub create_mode: Option<String>,
    /// Customer managed key status
    #[serde(alias = "customerManagedKeyStatus")]
    pub customer_managed_key_status: Option<String>,
    /// Database account offer type
    #[serde(alias = "databaseAccountOfferType")]
    pub database_account_offer_type: Option<String>,
    /// Default identity
    #[serde(alias = "defaultIdentity")]
    pub default_identity: Option<String>,
    /// Whether key-based metadata write access is disabled
    #[serde(alias = "disableKeyBasedMetadataWriteAccess")]
    pub disable_key_based_metadata_write_access: Option<bool>,
    /// Whether local authentication is disabled
    #[serde(alias = "disableLocalAuth")]
    pub disable_local_auth: Option<bool>,
    /// Whether analytical storage is enabled
    #[serde(alias = "enableAnalyticalStorage")]
    pub enable_analytical_storage: Option<bool>,
    /// Whether automatic failover is enabled
    #[serde(alias = "enableAutomaticFailover")]
    pub enable_automatic_failover: Option<bool>,
    /// Whether burst capacity is enabled
    #[serde(alias = "enableBurstCapacity")]
    pub enable_burst_capacity: Option<bool>,
    /// Whether Cassandra connector is enabled
    #[serde(alias = "enableCassandraConnector")]
    pub enable_cassandra_connector: Option<bool>,
    /// Whether free tier is enabled
    #[serde(alias = "enableFreeTier")]
    pub enable_free_tier: Option<bool>,
    /// Whether multiple write locations are enabled
    #[serde(alias = "enableMultipleWriteLocations")]
    pub enable_multiple_write_locations: Option<bool>,
    /// Whether partition merge is enabled
    #[serde(alias = "enablePartitionMerge")]
    pub enable_partition_merge: Option<bool>,
    /// Whether per-region per-partition autoscale is enabled
    #[serde(alias = "enablePerRegionPerPartitionAutoscale")]
    pub enable_per_region_per_partition_autoscale: Option<bool>,
    /// Failover policies
    #[serde(alias = "failoverPolicies")]
    pub failover_policies: Option<Vec<FailoverPolicy>>,
    /// Identity configuration
    pub identity: Option<AccountIdentity>,
    /// Instance ID
    #[serde(alias = "instanceId")]
    pub instance_id: Option<String>,
    /// IP rules
    #[serde(alias = "ipRules")]
    pub ip_rules: Option<Vec<IpRule>>,
    /// Whether virtual network filter is enabled
    #[serde(alias = "isVirtualNetworkFilterEnabled")]
    pub is_virtual_network_filter_enabled: Option<bool>,
    /// Key Vault key URI
    #[serde(alias = "keyVaultKeyUri")]
    pub key_vault_key_uri: Option<String>,
    /// Keys metadata
    #[serde(alias = "keysMetadata")]
    pub keys_metadata: Option<KeysMetadata>,
    /// Account locations
    pub locations: Option<Vec<AccountLocation>>,
    /// Minimal TLS version
    #[serde(alias = "minimalTlsVersion")]
    pub minimal_tls_version: Option<String>,
    /// Network ACL bypass
    #[serde(alias = "networkAclBypass")]
    pub network_acl_bypass: Option<String>,
    /// Resource IDs for network ACL bypass
    #[serde(alias = "networkAclBypassResourceIds")]
    pub network_acl_bypass_resource_ids: Option<Vec<String>>,
    /// Private endpoint connections
    #[serde(alias = "privateEndpointConnections")]
    pub private_endpoint_connections: Option<Vec<PrivateEndpointConnection>>,
    /// Public network access
    #[serde(alias = "publicNetworkAccess")]
    pub public_network_access: Option<String>,
    /// Read locations
    #[serde(alias = "readLocations")]
    pub read_locations: Option<Vec<AccountLocation>>,
    /// Restore parameters
    #[serde(alias = "restoreParameters")]
    pub restore_parameters: Option<RestoreParameters>,
    /// System data
    #[serde(alias = "systemData")]
    pub system_data: Option<SystemData>,
    /// Virtual network rules
    #[serde(alias = "virtualNetworkRules")]
    pub virtual_network_rules: Option<Vec<VirtualNetworkRule>>,
    /// Write locations
    #[serde(alias = "writeLocations")]
    pub write_locations: Option<Vec<AccountLocation>>,
}

/// Analytical storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyticalStorageConfiguration {
    /// Schema type
    pub schema_type: Option<String>,
}

/// API properties
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiProperties {
    /// Server version (for MongoDB)
    pub server_version: Option<String>,
}

/// Account capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    /// Capability name
    pub name: String,
}

/// Backup policy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupPolicy {
    /// Backup type
    #[serde(rename = "type")]
    pub backup_type: String,
    /// Migration state
    pub migration_state: Option<String>,
    /// Periodic mode properties
    pub periodic_mode_properties: Option<PeriodicBackupProperties>,
    /// Continuous mode properties
    pub continuous_mode_properties: Option<ContinuousBackupProperties>,
}

/// Periodic backup properties
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeriodicBackupProperties {
    /// Backup interval in minutes
    pub backup_interval_in_minutes: Option<i32>,
    /// Backup retention interval in hours
    pub backup_retention_interval_in_hours: Option<i32>,
    /// Backup storage redundancy
    pub backup_storage_redundancy: Option<String>,
}

/// Continuous backup properties
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinuousBackupProperties {
    /// Continuous backup tier (e.g., Continuous7Days, Continuous30Days)
    pub tier: Option<String>,
}

/// Failover policy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FailoverPolicy {
    /// Failover priority
    pub failover_priority: i32,
    /// Location ID
    pub id: Option<String>,
    /// Location name
    pub location_name: String,
}

/// Account identity
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountIdentity {
    /// Principal ID
    pub principal_id: Option<String>,
    /// Tenant ID
    pub tenant_id: Option<String>,
    /// Identity type
    #[serde(rename = "type")]
    pub identity_type: Option<String>,
    /// User assigned identities
    pub user_assigned_identities: Option<HashMap<String, serde_json::Value>>,
}

/// IP rule
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IpRule {
    /// IP address or range
    pub ip_address_or_range: String,
}

/// Keys metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeysMetadata {
    /// Primary master key
    pub primary_master_key: Option<KeyMetadata>,
    /// Primary readonly master key
    pub primary_readonly_master_key: Option<KeyMetadata>,
    /// Secondary master key
    pub secondary_master_key: Option<KeyMetadata>,
    /// Secondary readonly master key
    pub secondary_readonly_master_key: Option<KeyMetadata>,
}

/// Key metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyMetadata {
    /// Generation time
    pub generation_time: Option<String>,
}

/// Private endpoint connection
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivateEndpointConnection {
    /// Connection ID
    pub id: Option<String>,
    /// Connection name
    pub name: Option<String>,
    /// Connection type
    #[serde(rename = "type")]
    pub connection_type: Option<String>,
    /// Group ID
    pub group_id: Option<String>,
    /// Private endpoint
    pub private_endpoint: Option<PrivateEndpoint>,
    /// Private link service connection state
    pub private_link_service_connection_state: Option<PrivateLinkServiceConnectionState>,
    /// Provisioning state
    pub provisioning_state: Option<String>,
    /// Resource group
    pub resource_group: Option<String>,
}

/// Private endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivateEndpoint {
    /// Private endpoint ID
    pub id: String,
    /// Resource group
    pub resource_group: Option<String>,
}

/// Private link service connection state
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivateLinkServiceConnectionState {
    /// Actions required
    pub actions_required: Option<String>,
    /// Description
    pub description: Option<String>,
    /// Status
    pub status: Option<String>,
}

/// Restore parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RestoreParameters {
    /// Restore mode
    pub restore_mode: Option<String>,
    /// Restore source
    pub restore_source: Option<String>,
    /// Restore timestamp in UTC
    pub restore_timestamp_in_utc: Option<String>,
}

/// System data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemData {
    /// Creation date
    pub created_at: Option<String>,
    /// Created by
    pub created_by: Option<String>,
    /// Created by type
    pub created_by_type: Option<String>,
    /// Last modification date
    pub last_modified_at: Option<String>,
    /// Last modified by
    pub last_modified_by: Option<String>,
    /// Last modified by type
    pub last_modified_by_type: Option<String>,
}

/// Virtual network rule
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualNetworkRule {
    /// Virtual network ID
    pub id: String,
    /// Whether to ignore missing VNet service endpoint
    pub ignore_missing_vnet_service_endpoint: Option<bool>,
    /// Resource group
    pub resource_group: Option<String>,
}

/// Capacity settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapacitySettings {
    /// Total provisioned throughput limit
    pub total_throughput_limit: Option<i32>,
}

/// Consistency policy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsistencyPolicy {
    /// Default consistency level
    pub default_consistency_level: String,
    /// Maximum tolerated staleness
    pub max_staleness_prefix: Option<i64>,
    /// Maximum staleness interval
    pub max_interval_in_seconds: Option<i32>,
}

/// CORS policy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CorsPolicy {
    /// Allowed origins
    pub allowed_origins: String,
    /// Allowed methods
    pub allowed_methods: String,
    /// Allowed headers
    pub allowed_headers: String,
    /// Exposed headers
    pub exposed_headers: String,
    /// Maximum age
    pub max_age_in_seconds: Option<i32>,
}

/// Account location
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountLocation {
    /// Location name
    pub location_name: String,
    /// Provisioning status
    pub provisioning_state: String,
    /// Failover status
    pub failover_priority: i32,
    /// Whether it's a write region
    pub is_zone_redundant: Option<bool>,
    /// Location ID
    pub id: Option<String>,
    /// Document endpoint URI
    pub document_endpoint: Option<String>,
}

/// Cosmos DB Database information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosDatabase {
    /// Database name
    pub name: String,
    /// Database ID
    pub id: String,
    /// Throughput settings
    pub throughput_settings: Option<ThroughputSettings>,
}

/// Throughput settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThroughputSettings {
    /// Manual throughput
    pub throughput: Option<i32>,
    /// Autoscale settings
    pub autoscale_settings: Option<AutoscaleSettings>,
}

/// Autoscale settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoscaleSettings {
    /// Maximum throughput
    pub max_throughput: i32,
}

/// Cosmos DB Container information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosContainer {
    /// Container name
    pub name: String,
    /// Container ID
    pub id: String,
    /// Partition key
    pub partition_key: Option<PartitionKey>,
    /// Throughput settings
    pub throughput_settings: Option<ThroughputSettings>,
    /// Indexing policy
    pub indexing_policy: Option<IndexingPolicy>,
}

/// Partition key
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartitionKey {
    /// Partition key paths
    pub paths: Vec<String>,
    /// Partition key type
    pub kind: String,
}

/// Indexing policy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexingPolicy {
    /// Whether automatic indexing is enabled
    pub automatic: Option<bool>,
    /// Indexing mode
    pub indexing_mode: Option<String>,
    /// Included paths
    pub included_paths: Option<Vec<IndexPath>>,
    /// Excluded paths
    pub excluded_paths: Option<Vec<IndexPath>>,
}

/// Index path
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexPath {
    /// Path
    pub path: String,
}

/// Cosmos Account access keys
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosKeys {
    /// Primary master key
    pub primary_master_key: String,
    /// Secondary master key
    pub secondary_master_key: String,
    /// Primary readonly master key
    pub primary_readonly_master_key: String,
    /// Secondary readonly master key
    pub secondary_readonly_master_key: String,
}

/// Cosmos Account connection strings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosConnectionStrings {
    /// Connection strings
    pub connection_strings: Vec<ConnectionString>,
}

/// Individual connection string
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionString {
    /// Connection description
    pub description: String,
    /// Connection string
    pub connection_string: String,
} 