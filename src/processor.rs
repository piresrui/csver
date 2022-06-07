use crate::domain::datastore::DataStore;
use crate::domain::processor::{Processor, ProcessorError, ProcessorResult};
use crate::model::account::Account;
use crate::model::transaction::{Transaction, TxType};

pub struct TxProcessor<S: DataStore> {
    datastore: S,
}

impl<S: DataStore> Processor for TxProcessor<S> {
    fn process(&mut self, tx: Transaction) -> ProcessorResult<Account> {
        // We only need to store deposit and withdrawl since other types will just be notifying us
        if tx.tx_type == TxType::Deposit || tx.tx_type == TxType::Withdrawal {
            self.datastore.insert_tx(tx.clone()).unwrap();
        }

        let mut acc = self.datastore.get_account(tx.client).unwrap();

        if acc.locked {
            return Err(ProcessorError::AccountLocked);
        }

        self.apply(&mut acc, &tx).unwrap();

        self.datastore.update_account(acc.clone()).unwrap();
        Ok(acc)
    }

    fn accounts(&self) -> ProcessorResult<Vec<Account>> {
        Ok(self.datastore.get_accounts().unwrap())
    }
}

impl<S: DataStore> TxProcessor<S> {
    pub fn new(datastore: S) -> Self {
        Self { datastore }
    }

    fn apply(&mut self, acc: &mut Account, tx: &Transaction) -> ProcessorResult<()> {
        match tx.tx_type {
            TxType::Deposit => self.deposit(acc, &tx),
            TxType::Withdrawal => self.withdrawal(acc, &tx),
            TxType::Dispute => self.dispute(acc, &tx),
            TxType::Resolve => self.resolve(acc, &tx),
            TxType::ChargeBack => self.chargeback(acc, &tx),
        }
    }

    fn deposit(&self, account: &mut Account, tx: &Transaction) -> ProcessorResult<()> {
        account.available += tx.amount.unwrap();
        account.total += tx.amount.unwrap();
        Ok(())
    }

    fn withdrawal(&self, account: &mut Account, tx: &Transaction) -> ProcessorResult<()> {
        if account.available < tx.amount.unwrap() {
            return Err(ProcessorError::NoFunds);
        }

        account.available -= tx.amount.unwrap();
        account.total -= tx.amount.unwrap();

        Ok(())
    }

    fn dispute(&mut self, account: &mut Account, tx: &Transaction) -> ProcessorResult<()> {
        let tx = self.datastore.get_tx(tx.tx).unwrap();
        account.available -= tx.amount.unwrap();
        account.held += tx.amount.unwrap();

        if let Err(e) = self.datastore.mark_disputed(tx.tx) {
            print!("Failed to mark tx as disputed - {}", e)
        }
        Ok(())
    }

    fn resolve(&mut self, account: &mut Account, tx: &Transaction) -> ProcessorResult<()> {
        let tx = self.datastore.get_tx(tx.tx).unwrap();
        account.available += tx.amount.unwrap();
        account.held -= tx.amount.unwrap();

        if let Err(e) = self.datastore.mark_resolved(tx.tx) {
            print!("Failed to mark tx as resolved - {}", e)
        }
        Ok(())
    }

    fn chargeback(&mut self, account: &mut Account, tx: &Transaction) -> ProcessorResult<()> {
        let tx = self.datastore.get_tx(tx.tx).unwrap();
        account.held -= tx.amount.unwrap();
        account.total -= tx.amount.unwrap();
        account.locked = true;

        if let Err(e) = self.datastore.mark_resolved(tx.tx) {
            print!("Failed to mark tx as resolved - {}", e)
        }
        Ok(())
    }
}

#[cfg(tests)]
mod tests {
    use super::*;
    #[test]
    fn test_deposit() {
        let ds = MemStore::new();
        let p = TxProcessor::new(ds);
        let mut acc = Account::new(0);
        let mut tx = Transaction::default();
        tx.amount = decimal_rs::Decimal::from(10);

        let r = p.deposit(&mut acc, &tx);
        assert!(r.is_ok());
        assert_eq!(tx.amount, acc.total);
    }
    #[test]
    fn test_withdrawl() {
        let ds = MemStore::new();
        let p = TxProcessor::new(ds);
        let mut acc = Account::new(0);
        let mut tx = Transaction::default();
        tx.amount = decimal_rs::Decimal::from(10);
        let r = p.deposit(&mut acc, &tx);
        assert!(r.is_ok());
        assert_eq!(tx.amount, acc.total);

        let r = p.withdrawal(&mut acc, &tx);
        assert!(r.is_ok());
        assert_eq!(tx.amount, acc.total);
    }
    #[test]
    fn test_dispute() {
        let ds = MemStore::new();
        let mut p = TxProcessor::new(ds);
        let mut acc = Account::new(0);
        let tx = Transaction::default();

        let r = p.datastore.insert_tx(tx.clone());
        assert!(r.is_ok());

        let result = p.dispute(&mut acc, &tx);
        assert!(result.is_ok());

        assert!(p.datastore.get_tx(tx.tx).unwrap().disputed);
    }
    #[test]
    fn test_resolved() {
        let ds = MemStore::new();
        let mut p = TxProcessor::new(ds);
        let mut acc = Account::new(0);
        let tx = Transaction::default();

        let r = p.datastore.insert_tx(tx.clone());
        assert!(r.is_ok());

        let mut result = p.dispute(&mut acc, &tx);
        assert!(result.is_ok());

        assert!(p.datastore.get_tx(tx.tx).unwrap().disputed);
        result = p.resolve(&mut acc, &tx);
        assert!(result.is_ok());

        assert!(!p.datastore.get_tx(tx.tx).unwrap().disputed);
    }

    #[test]
    fn test_process() {
        let ds = MemStore::new();
        let mut p = TxProcessor::new(ds);
        let mut tx = Transaction::default();
        tx.amount = decimal_rs::Decimal::from(10);

        let r = p.process(tx);
        assert!(r.is_ok());
        let acc = r.unwrap();
        assert_eq!(acc.client, 0);
        assert_eq!(acc.available, decimal_rs::Decimal::from(10));
    }
}
