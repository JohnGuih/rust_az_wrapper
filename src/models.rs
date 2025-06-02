//! Modelos de dados para recursos do Azure focados em Cosmos DB

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Informações de uma Subscription do Azure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    /// ID da subscription
    pub id: String,
    /// Nome da subscription
    #[serde(alias = "name")]
    pub display_name: String,
    /// Estado da subscription
    pub state: String,
    /// ID do tenant
    #[serde(alias = "tenantId")]
    pub tenant_id: String,
    /// Se é a subscription padrão
    #[serde(alias = "isDefault")]
    pub is_default: Option<bool>,
}

/// Informações de uma Cosmos DB Account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmosAccount {
    /// Nome da conta
    pub name: String,
    /// ID do recurso
    pub id: String,
    /// Localização da conta
    pub location: String,
    /// Grupo de recursos
    #[serde(alias = "resourceGroup", alias = "resource_group")]
    pub resource_group: String,
    /// Tipo de recurso
    #[serde(rename = "type", alias = "resourceType", alias = "resource_type")]
    pub resource_type: String,
    /// Tipo de conta (SQL, MongoDB, etc.)
    pub kind: String,
    /// Status de provisionamento
    #[serde(alias = "provisioningState", alias = "provisioning_state")]
    pub provisioning_state: String,
    /// URI de endpoint
    #[serde(alias = "documentEndpoint", alias = "document_endpoint")]
    pub document_endpoint: String,
    /// Tags associadas
    pub tags: Option<HashMap<String, String>>,
    /// Propriedades específicas
    pub properties: Option<CosmosAccountProperties>,
}

/// Propriedades detalhadas de uma Cosmos Account
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosAccountProperties {
    /// Configurações de backup
    pub backup_policy: Option<BackupPolicy>,
    /// Configurações de capacidade
    pub capacity: Option<CapacitySettings>,
    /// Configurações de consistência
    pub consistency_policy: Option<ConsistencyPolicy>,
    /// Configurações de CORS
    pub cors: Option<Vec<CorsPolicy>>,
    /// Se failover automático está habilitado
    pub enable_automatic_failover: Option<bool>,
    /// Se multiple write locations está habilitado
    pub enable_multiple_write_locations: Option<bool>,
    /// Localizações da conta
    pub locations: Option<Vec<AccountLocation>>,
}

/// Política de backup
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupPolicy {
    /// Tipo de backup
    #[serde(rename = "type")]
    pub backup_type: String,
    /// Se backup periódico está habilitado
    pub periodic_mode_properties: Option<PeriodicBackupProperties>,
}

/// Propriedades de backup periódico
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeriodicBackupProperties {
    /// Intervalo de backup em minutos
    pub backup_interval_in_minutes: Option<i32>,
    /// Retenção de backup em horas
    pub backup_retention_interval_in_hours: Option<i32>,
}

/// Configurações de capacidade
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapacitySettings {
    /// Throughput total provisionado
    pub total_throughput_limit: Option<i32>,
}

/// Política de consistência
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsistencyPolicy {
    /// Nível de consistência padrão
    pub default_consistency_level: String,
    /// Staleness máximo tolerado
    pub max_staleness_prefix: Option<i64>,
    /// Intervalo máximo de staleness
    pub max_interval_in_seconds: Option<i32>,
}

/// Política de CORS
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CorsPolicy {
    /// Origens permitidas
    pub allowed_origins: String,
    /// Métodos permitidos
    pub allowed_methods: String,
    /// Headers permitidos
    pub allowed_headers: String,
    /// Headers expostos
    pub exposed_headers: String,
    /// Idade máxima
    pub max_age_in_seconds: Option<i32>,
}

/// Localização de uma account
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountLocation {
    /// Nome da localização
    pub location_name: String,
    /// Status de provisionamento
    pub provisioning_state: String,
    /// Status de failover
    pub failover_priority: i32,
    /// Se é região de escrita
    pub is_zone_redundant: Option<bool>,
}

/// Informações de uma Database do Cosmos DB
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosDatabase {
    /// Nome da database
    pub name: String,
    /// ID da database
    pub id: String,
    /// Configurações de throughput
    pub throughput_settings: Option<ThroughputSettings>,
}

/// Configurações de throughput
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThroughputSettings {
    /// Throughput manual
    pub throughput: Option<i32>,
    /// Configurações de autoscale
    pub autoscale_settings: Option<AutoscaleSettings>,
}

/// Configurações de autoscale
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoscaleSettings {
    /// Throughput máximo
    pub max_throughput: i32,
}

/// Informações de um Container do Cosmos DB
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosContainer {
    /// Nome do container
    pub name: String,
    /// ID do container
    pub id: String,
    /// Chave de partição
    pub partition_key: Option<PartitionKey>,
    /// Configurações de throughput
    pub throughput_settings: Option<ThroughputSettings>,
    /// Política de indexação
    pub indexing_policy: Option<IndexingPolicy>,
}

/// Chave de partição
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartitionKey {
    /// Caminhos da chave de partição
    pub paths: Vec<String>,
    /// Tipo da chave de partição
    pub kind: String,
}

/// Política de indexação
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexingPolicy {
    /// Se indexação automática está habilitada
    pub automatic: Option<bool>,
    /// Modo de indexação
    pub indexing_mode: Option<String>,
    /// Caminhos incluídos
    pub included_paths: Option<Vec<IndexPath>>,
    /// Caminhos excluídos
    pub excluded_paths: Option<Vec<IndexPath>>,
}

/// Caminho de indexação
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexPath {
    /// Caminho
    pub path: String,
}

/// Chaves de acesso de uma Cosmos Account
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosKeys {
    /// Chave primária master
    pub primary_master_key: String,
    /// Chave secundária master
    pub secondary_master_key: String,
    /// Chave primária readonly
    pub primary_readonly_master_key: String,
    /// Chave secundária readonly
    pub secondary_readonly_master_key: String,
}

/// Strings de conexão de uma Cosmos Account
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CosmosConnectionStrings {
    /// Strings de conexão
    pub connection_strings: Vec<ConnectionString>,
}

/// String de conexão individual
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionString {
    /// Descrição da conexão
    pub description: String,
    /// String de conexão
    pub connection_string: String,
} 