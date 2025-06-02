//! Testes de integração para o wrapper Azure CLI
//! 
//! Estes testes requerem que o Azure CLI esteja instalado e configurado.
//! Execute com: cargo test --test integration_tests -- --ignored

use rust_az_wrapper::{AzureClient, Result};

#[tokio::test]
#[ignore] // Ignora por padrão pois requer Azure CLI configurado
async fn test_list_subscriptions() -> Result<()> {
    let client = AzureClient::new()?;
    
    let subscriptions = client.list_subscriptions().await?;
    
    // Deve ter pelo menos uma subscription
    assert!(!subscriptions.is_empty(), "Deve ter pelo menos uma subscription");
    
    // Verifica se os campos obrigatórios estão presentes
    for sub in &subscriptions {
        assert!(!sub.id.is_empty(), "ID da subscription não pode estar vazio");
        assert!(!sub.display_name.is_empty(), "Nome da subscription não pode estar vazio");
        assert!(!sub.tenant_id.is_empty(), "Tenant ID não pode estar vazio");
    }
    
    println!("✅ Teste de listagem de subscriptions passou");
    Ok(())
}

#[tokio::test]
#[ignore] // Ignora por padrão pois requer Azure CLI configurado
async fn test_show_current_subscription() -> Result<()> {
    let client = AzureClient::new()?;
    
    let subscription = client.show_current_subscription().await?;
    
    // Verifica se os campos obrigatórios estão presentes
    assert!(!subscription.id.is_empty(), "ID da subscription não pode estar vazio");
    assert!(!subscription.display_name.is_empty(), "Nome da subscription não pode estar vazio");
    assert!(!subscription.tenant_id.is_empty(), "Tenant ID não pode estar vazio");
    
    println!("✅ Teste de subscription atual passou");
    Ok(())
}

#[tokio::test]
#[ignore] // Ignora por padrão pois requer Azure CLI configurado
async fn test_list_cosmos_accounts() -> Result<()> {
    let client = AzureClient::new()?;
    
    let accounts = client.list_cosmos_accounts(None).await?;
    
    // Pode não ter accounts, mas se tiver, deve ter campos válidos
    for account in &accounts {
        assert!(!account.name.is_empty(), "Nome da account não pode estar vazio");
        assert!(!account.id.is_empty(), "ID da account não pode estar vazio");
        assert!(!account.location.is_empty(), "Localização não pode estar vazia");
        assert!(!account.resource_group.is_empty(), "Resource group não pode estar vazio");
        assert!(!account.document_endpoint.is_empty(), "Document endpoint não pode estar vazio");
    }
    
    println!("✅ Teste de listagem de Cosmos accounts passou");
    Ok(())
}

#[tokio::test]
#[ignore] // Ignora por padrão pois requer Azure CLI configurado
async fn test_json_serialization() -> Result<()> {
    let client = AzureClient::new()?;
    
    // Testa serialização de subscriptions
    let subscriptions = client.list_subscriptions().await?;
    let json = serde_json::to_string_pretty(&subscriptions)?;
    assert!(!json.is_empty(), "JSON de subscriptions não pode estar vazio");
    
    // Testa deserialização
    let _deserialized: Vec<rust_az_wrapper::Subscription> = serde_json::from_str(&json)?;
    
    // Testa serialização de Cosmos accounts
    let accounts = client.list_cosmos_accounts(None).await?;
    let json = serde_json::to_string_pretty(&accounts)?;
    assert!(!json.is_empty(), "JSON de Cosmos accounts não pode estar vazio");
    
    // Testa deserialização
    let _deserialized: Vec<rust_az_wrapper::CosmosAccount> = serde_json::from_str(&json)?;
    
    println!("✅ Teste de serialização JSON passou");
    Ok(())
}

#[tokio::test]
#[ignore] // Ignora por padrão pois requer Azure CLI configurado
async fn test_cosmos_operations_if_account_exists() -> Result<()> {
    let client = AzureClient::new()?;
    
    let accounts = client.list_cosmos_accounts(None).await?;
    
    if let Some(account) = accounts.first() {
        let account_name = &account.name;
        let resource_group = &account.resource_group;
        
        // Testa obtenção de chaves
        let keys = client.list_cosmos_keys(account_name, resource_group).await?;
        assert!(!keys.primary_master_key.is_empty(), "Primary key não pode estar vazia");
        assert!(!keys.secondary_master_key.is_empty(), "Secondary key não pode estar vazia");
        
        // Testa listagem de databases SQL
        let databases = client.list_sql_databases(account_name, resource_group).await?;
        
        // Se há databases, testa listagem de containers
        if let Some(database) = databases.first() {
            let containers = client.list_sql_containers(account_name, resource_group, &database.name).await?;
            
            // Verifica se containers têm campos válidos
            for container in &containers {
                assert!(!container.name.is_empty(), "Nome do container não pode estar vazio");
                assert!(!container.id.is_empty(), "ID do container não pode estar vazio");
            }
        }
        
        println!("✅ Teste de operações Cosmos passou");
    } else {
        println!("⚠️ Nenhuma Cosmos account encontrada, pulando teste de operações");
    }
    
    Ok(())
} 