use crate::model::transaction::{Amount, ClientID};
use decimal_rs::Decimal;
use serde::{Deserialize, Serialize};

const DISPLAY_PRECISION: i16 = 4;

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
            available: Decimal::ZERO,
            held: Decimal::ZERO,
            total: Decimal::ZERO,
            locked: false,
        }
    }

    pub fn scale(&mut self) {
        self.available.normalize_to_scale(DISPLAY_PRECISION);
        self.held.normalize_to_scale(DISPLAY_PRECISION);
        self.total.normalize_to_scale(DISPLAY_PRECISION);
    }
}
