mod datastore;
mod domain;
mod model;
mod processor;
mod reader_writer;

use datastore::MemStore;
use domain::processor::Processor;
use model::transaction::Transaction;
use processor::TxProcessor;

fn main() {
    process();
}

fn process() {
    let store = MemStore::new();
    let mut processor = TxProcessor::new(store);

    for result in reader_writer::csv_reader("test.csv".to_string()).deserialize() {
        let tx: Transaction = result.unwrap();
        if let Err(e) = processor.process(tx) {
            println!("Error processing transaction - {}", e)
        }
    }
}
