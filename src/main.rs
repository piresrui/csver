mod datastore;
mod domain;
mod model;
mod processor;
mod reader_writer;

use datastore::MemStore;
use domain::processor::Processor;
use model::transaction::Transaction;
use processor::TxProcessor;
use std::error::Error;

fn main() {
    match process() {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}

fn process() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let store = MemStore::new();
    let mut processor = TxProcessor::new(store);
    let mut reader = reader_writer::csv_reader(args[1].clone());
    for result in reader.deserialize() {
        let tx: Transaction = result?;
        if let Err(e) = processor.process(tx) {
            println!("Error processing transaction - {}", e);
        }
    }

    let accs = processor.accounts().unwrap();
    let mut writer = reader_writer::stdout_writer();
    accs.into_iter().for_each(|account| {
        let mut acc = account.clone();
        acc.scale();
        if let Err(e) = writer.serialize(acc) {
            println!("Error serializing - {}", e);
        }
    });
    Ok(())
}
