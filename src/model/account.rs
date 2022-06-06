use crate::model::transaction::{Amount, ClientID, Transaction, TxID};
use serde::{Deserialize, Serialize};

/// Account contains the state for a client account
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Account {
    /// id of client
    pub client: ClientID,
    /// available amount for client
    pub available: Amount,
    /// held amount for client
    pub held: Amount,
    /// total amount for client
    pub total: Amount,
    /// is client account locked
    pub locked: bool,
}

impl Account {
    pub fn new(id: ClientID) -> Self {
        Account {
            client: id,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        }
    }
}
