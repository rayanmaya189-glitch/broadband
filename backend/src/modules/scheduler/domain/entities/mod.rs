pub mod job_definition;
pub mod job_execution;

pub use job_definition::ActiveModel as JobDefinitionActiveModel;
pub use job_definition::Entity as JobDefinition;

pub use job_execution::ActiveModel as JobExecutionActiveModel;
pub use job_execution::Entity as JobExecution;
