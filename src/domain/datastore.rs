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
    #[error("Account not found in store")]
    AccountNotFound,
    #[error("Account already exists in store")]
    AccountAlreadyExists,
}

// Result type for datastore
pub type DataStoreResult<T> = Result<T, DataStoreError>;

// DataStore is in charge of providing an interface for interacting with transaction/account state
pub trait DataStore {
    fn get_tx(&self, id: TxID) -> DataStoreResult<Transaction>;
    fn insert_tx(&mut self, tx: Transaction) -> DataStoreResult<Transaction>;
    fn get_account(&self, id: ClientID) -> DataStoreResult<Account>;
    fn insert_account(&mut self, acc: Account) -> DataStoreResult<Account>;
}
