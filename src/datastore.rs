use serde::{Deserialize, Serialize};
use std::collections::hash_map::{Entry, HashMap};
use thiserror::Error;

type TxID = u32;
type ClientID = u16;
type Amount = f32;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
enum TxType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    ChargeBack,
}

#[derive(Debug, Deserialize, Clone)]
struct Transaction {
    #[serde(rename = "type")]
    tx_type: TxType,
    client: ClientID,
    tx: TxID,
    amount: Amount,
}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            tx_type: TxType::Deposit,
            client: 0,
            tx: 0,
            amount: 0.0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Account {
    client: ClientID,
    available: Amount,
    held: Amount,
    total: Amount,
    locked: bool,
    #[serde(skip_serializing)]
    txs: Vec<TxID>,
}

impl Account {
    fn new(id: ClientID) -> Self {
        Account {
            client: id,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
            txs: Vec::default(),
        }
    }

    fn add_tx(&mut self, tx: Transaction) {
        self.txs.push(tx.tx);
    }
}

struct TxEngine<S: DataStore> {
    datastore: S,
}

struct MemStore {
    accounts: HashMap<ClientID, Account>,
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

#[derive(Error, Debug)]
enum DataStoreError {
    #[error("Transaction already exists in store")]
    TxAlreadyExists,
    #[error("Transaction not found in store")]
    TxNotFound,
}

type DataStoreResult<T> = Result<T, DataStoreError>;

trait DataStore {
    fn get_tx(&self, id: TxID) -> DataStoreResult<Transaction>;
    fn insert_tx(&mut self, tx: Transaction) -> DataStoreResult<()>;
    fn get_account(&self, id: ClientID) -> DataStoreResult<Account>;
}

trait Engine {}

impl DataStore for MemStore {
    fn get_tx(&self, id: TxID) -> DataStoreResult<Transaction> {
        match self.txs.get(&id).cloned() {
            Some(t) => Ok(t),
            None => Err(DataStoreError::TxNotFound),
        }
    }

    fn insert_tx(&mut self, tx: Transaction) -> DataStoreResult<()> {
        match self.txs.entry(tx.tx) {
            Entry::Vacant(v) => {
                v.insert(tx);
                Ok(())
            }
            Entry::Occupied(_) => Err(DataStoreError::TxAlreadyExists),
        }
    }

    fn get_account(&self, id: ClientID) -> DataStoreResult<Account> {
        todo!()
    }
}

impl<S: DataStore> Engine for TxEngine<S> {}

mod tests {
    use super::*;

    #[test]
    fn test_insert_tx_account() {
        let acc = Account::new(0);
        let mut store = MemStore::new();
        let tx = Transaction::default();

        store.insert_tx(tx.clone());

        let got_tx = store.get_tx(tx.tx);
        assert_eq!(0, got_tx.unwrap().tx);
    }
}
