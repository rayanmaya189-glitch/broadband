use async_trait::async_trait;
use std::fmt::Debug;

use crate::shared::errors::AppError;

/// Marker trait for commands (write operations)
pub trait Command: Send + Sync + Debug {
    /// The type of result this command produces
    type Result: Send;

    /// Validate the command before execution
    fn validate(&self) -> Result<(), AppError>;
}

/// Marker trait for queries (read operations)
pub trait Query: Send + Sync + Debug {
    /// The type of result this query produces
    type Result: Send;
}

/// Handler for commands
#[async_trait]
pub trait CommandHandler<C: Command>: Send + Sync {
    /// Handle the command and produce a result
    async fn handle(&self, command: C) -> Result<C::Result, AppError>;
}

/// Handler for queries
#[async_trait]
pub trait QueryHandler<Q: Query>: Send + Sync {
    /// Handle the query and return results
    async fn handle(&self, query: Q) -> Result<Q::Result, AppError>;
}

/// Command bus for dispatching commands
#[async_trait]
pub trait CommandBus: Send + Sync {
    /// Dispatch a command to its handler
    async fn dispatch<C: Command>(&self, command: C) -> Result<C::Result, AppError>;
}

/// Query bus for dispatching queries
#[async_trait]
pub trait QueryBus: Send + Sync {
    /// Dispatch a query to its handler
    async fn dispatch<Q: Query>(&self, query: Q) -> Result<Q::Result, AppError>;
}

/// Repository trait for aggregates
#[async_trait]
pub trait Repository<T, Id>: Send + Sync {
    /// Find an aggregate by its ID
    async fn find_by_id(&self, id: Id) -> Result<Option<T>, AppError>;

    /// Save an aggregate (create or update)
    async fn save(&self, aggregate: &T) -> Result<(), AppError>;

    /// Delete an aggregate
    async fn delete(&self, id: Id) -> Result<(), AppError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct TestCommand {
        name: String,
    }

    impl Command for TestCommand {
        type Result = String;

        fn validate(&self) -> Result<(), AppError> {
            if self.name.is_empty() {
                return Err(AppError::Validation("Name cannot be empty".to_string()));
            }
            Ok(())
        }
    }

    #[test]
    fn test_command_validation() {
        let valid = TestCommand {
            name: "test".to_string(),
        };
        assert!(valid.validate().is_ok());

        let invalid = TestCommand {
            name: String::new(),
        };
        assert!(invalid.validate().is_err());
    }
}
