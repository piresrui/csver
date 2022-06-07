use crate::domain::datastore::{DataStore, DataStoreError, DataStoreResult};
use crate::model::account::Account;
use crate::model::transaction::{ClientID, Transaction, TxID};
use std::{collections::hash_map::Entry, collections::HashMap};

/// MemStore is an in memory datastore
pub struct MemStore {
    /// accounts contains a map of client id to account
    pub accounts: HashMap<ClientID, Account>,
    /// txs contains a map of transaction id to transaction
    pub txs: HashMap<TxID, Transaction>,
}

impl MemStore {
    pub fn new() -> Self {
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

    fn get_account(&mut self, id: ClientID) -> DataStoreResult<Account> {
        let acc = self.accounts.entry(id).or_insert(Account::new(id));
        Ok(acc.clone())
    }

    fn update_account(&mut self, acc: Account) -> DataStoreResult<()> {
        let account = acc.clone();
        *self.accounts.entry(acc.client).or_insert(acc) = account;
        Ok(())
    }

    fn mark_disputed(&mut self, id: TxID) -> DataStoreResult<()> {
        match self.txs.get_mut(&id) {
            Some(tx) => {
                tx.disputed = true;
                Ok(())
            }
            None => Ok(()),
        }
    }

    fn mark_resolved(&mut self, id: TxID) -> DataStoreResult<()> {
        match self.txs.get_mut(&id) {
            Some(tx) => {
                tx.disputed = false;
                Ok(())
            }
            None => Ok(()),
        }
    }

    fn get_accounts(&self) -> DataStoreResult<Vec<Account>> {
        Ok(self.accounts.values().cloned().collect::<Vec<_>>())
    }
}

#[cfg(test)]
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

        let got_acc = store.get_account(0);
        assert_eq!(acc, got_acc.unwrap());
    }
}
