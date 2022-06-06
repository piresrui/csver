use crate::domain::datastore::{DataStore, DataStoreError, DataStoreResult};
use crate::model::account::Account;
use crate::model::transaction::{ClientID, Transaction, TxID};
use std::collections::hash_map::{Entry, HashMap};

/// MemStore is an in memory datastore
struct MemStore {
    /// accounts contains a map of client id to account
    accounts: HashMap<ClientID, Account>,
    /// txs contains a map of transaction id to transaction
    txs: HashMap<TxID, Transaction>,
}

impl MemStore {
    fn new() -> Self {
        Self {
            accounts: HashMap::default(),
            txs: HashMap::default(),
        }
    }
}

impl DataStore for MemStore {
    fn get_tx(&self, id: TxID) -> DataStoreResult<Transaction> {
        match self.txs.get(&id).cloned() {
            Some(t) => Ok(t),
            None => Err(DataStoreError::TxNotFound),
        }
    }

    fn insert_tx(&mut self, tx: Transaction) -> DataStoreResult<Transaction> {
        match self.txs.entry(tx.tx) {
            Entry::Vacant(v) => {
                v.insert(tx.clone());
                Ok(tx)
            }
            Entry::Occupied(_) => Err(DataStoreError::TxAlreadyExists),
        }
    }

    fn get_account(&self, id: ClientID) -> DataStoreResult<Account> {
        match self.accounts.get(&id).cloned() {
            Some(acc) => Ok(acc),
            None => Err(DataStoreError::AccountNotFound),
        }
    }

    fn insert_account(&mut self, acc: Account) -> DataStoreResult<Account> {
        match self.accounts.entry(acc.client) {
            Entry::Vacant(v) => {
                v.insert(acc.clone());
                Ok(acc)
            }
            Entry::Occupied(_) => Err(DataStoreError::AccountAlreadyExists),
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_insert_tx_account() {
        let mut store = MemStore::new();
        let tx = Transaction::default();

        let r = store.insert_tx(tx.clone());
        assert!(r.is_ok());
        assert_eq!(tx, r.unwrap());

        let got_tx = store.get_tx(tx.tx);
        assert_eq!(0, got_tx.unwrap().tx);
    }

    #[test]
    fn test_insert_tx_already_exists() {
        let mut store = MemStore::new();
        let tx = Transaction::default();

        let mut r = store.insert_tx(tx.clone());
        assert!(r.is_ok());
        assert_eq!(tx, r.unwrap());

        r = store.insert_tx(tx.clone());
        assert!(r.is_err());
        assert_eq!(r.unwrap_err(), DataStoreError::TxAlreadyExists);
    }

    #[test]
    fn test_insert_account() {
        let mut store = MemStore::new();
        let acc = Account::new(0);

        let r = store.insert_account(acc.clone());
        assert!(r.is_ok());
        assert_eq!(acc, r.unwrap());

        let got_acc = store.get_account(0);
        assert_eq!(acc, got_acc.unwrap());
    }
    #[test]
    fn test_insert_account_already_exists() {
        let mut store = MemStore::new();
        let acc = Account::new(0);

        let mut r = store.insert_account(acc.clone());
        assert!(r.is_ok());
        assert_eq!(acc, r.unwrap());

        r = store.insert_account(acc.clone());
        assert!(r.is_err());
        assert_eq!(r.unwrap_err(), DataStoreError::AccountAlreadyExists);
    }
}
