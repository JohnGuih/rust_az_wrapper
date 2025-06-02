# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2024-XX-XX

### ✨ Added
- **Specific subscription support**: Now you can work with specific subscriptions instead of just the default Azure CLI subscription
- **New methods in AzureClient**:
  - `AzureClient::with_subscription(id)` - creates client configured for specific subscription
  - `use_subscription(id)` - sets specific subscription on existing client
  - `clear_subscription()` - returns to using default subscription
  - `get_subscription()` - gets configured subscription
- **New example**: `subscription_usage.rs` demonstrating specific subscription usage
- **--subscription flag** automatically added to all commands that support it

### 🔧 Modified
- **AzCommandBuilder**: New `subscription(Option<&str>)` method to add subscription flag
- **All command functions**: Now accept `subscription_id: Option<&str>` parameter
- **Error handling**: Improved error messages with command context
- **Client**: Internal subscription state management

### 📚 Updated
- **README**: Complete documentation in English, examples updated
- **Comments**: All code comments translated to English
- **Examples**: Updated to demonstrate new subscription functionality

### 🧪 Tested
- ✅ All unit tests passing
- ✅ Integration tests with real Azure data
- ✅ Examples tested with multiple subscriptions
- ✅ Backward compatibility maintained

## [0.1.0] - 2024-XX-XX

### ✨ Initial Release
- **Core Azure CLI wrapper** for Cosmos DB operations
- **Type-safe models** for all Azure resources
- **Complete Cosmos DB API coverage**:
  - Accounts (CRUD, failover, locations)
  - Keys and connection strings
  - SQL databases and containers
  - MongoDB databases and collections
  - Throughput management
- **Resource Groups** management
- **Subscription** operations
- **Automatic JSON serialization/deserialization**
- **Comprehensive error handling**
- **Async/await support** with Tokio
- **Integration tests** with real Azure data
- **Production-ready examples** 