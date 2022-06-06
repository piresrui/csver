use serde::Deserialize;

/// Wrapper on u32 represents transaction ID
pub type TxID = u32;
/// Wrapper on u16 represents client ID
pub type ClientID = u16;
/// Wrapper on f32 represents tx amount
pub type Amount = f32;

/// Enum containing transaction types
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TxType {
    /// Deposit tx
    Deposit,
    /// Withdrawal tx
    Withdrawal,
    /// Disputed tx
    Dispute,
    /// Resolved tx
    Resolve,
    /// Chargeback tx
    ChargeBack,
}

/// Transaction representation
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Transaction {
    /// type of transaction
    #[serde(rename = "type")]
    pub tx_type: TxType,
    /// id of client transaction belongs to
    pub client: ClientID,
    /// id of transaction
    pub tx: TxID,
    /// amount of transaction, only available for Deposit and Withdrawal tx
    pub amount: Amount,
    #[serde(skip_serializing)]
    pub disputed: bool,
}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            tx_type: TxType::Deposit,
            client: 0,
            tx: 0,
            amount: 0.0,
            disputed: false,
        }
    }
}
