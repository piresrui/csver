use crate::domain::processor::Engine;

struct TxEngine<S: DataStore> {
    datastore: S,
}

impl<S: DataStore> Engine for TxEngine<S> {}
