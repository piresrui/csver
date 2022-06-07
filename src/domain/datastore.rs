use crate::model::account::Account;
use crate::model::transaction::{ClientID, Transaction, TxID};
use thiserror::Error;

// Error type for datastore
#[derive(Error, Debug, PartialEq)]
pub enum DataStoreError {
    #[error("Transaction already exists in store")]
    TxAlreadyExists,
    #[error("Transaction not found in store")]
    TxNotFound,
}

// Result type for datastore
pub type DataStoreResult<T> = Result<T, DataStoreError>;

// DataStore is in charge of providing an interface for interacting with transaction/account state
pub trait DataStore {
    /// Returns clone of transaction
    /// Returns TxNotFound if no transaction
    fn get_tx(&self, id: TxID) -> DataStoreResult<Transaction>;
    /// Inserts transaction
    /// Retuns TxAlreadyExists if occupied
    fn insert_tx(&mut self, tx: Transaction) -> DataStoreResult<Transaction>;
    /// Returns clone of account
    /// Inserts if it does not exist
    fn get_account(&mut self, id: ClientID) -> DataStoreResult<Account>;
    /// Updates account
    fn update_account(&mut self, acc: Account) -> DataStoreResult<()>;
    /// Sets a transaction as disputed
    /// Does nothing if no Tx
    fn mark_disputed(&mut self, id: TxID) -> DataStoreResult<()>;
    /// Sets a transaction as resolved
    /// Does nothing if no Tx
    fn mark_resolved(&mut self, id: TxID) -> DataStoreResult<()>;
    /// Returns all accounts
    fn get_accounts(&self) -> DataStoreResult<Vec<Account>>;
}
