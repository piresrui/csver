use crate::model::account::Account;
use crate::model::transaction::Transaction;
use thiserror::Error;

// Processor errors
#[derive(Error, Debug, PartialEq)]
pub enum ProcessorError {
    #[error("Account is locked")]
    AccountLocked,
    #[error("No funds")]
    NoFunds,
}

// Result of processor operations
pub type ProcessorResult<T> = Result<T, ProcessorError>;

// Processor is responsible for defining behavior to deal with transactions
pub trait Processor {
    // Processes a given transaction
    fn process(&mut self, tx: Transaction) -> ProcessorResult<Account>;
}
