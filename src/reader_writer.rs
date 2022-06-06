use csv::{Reader, Trim};
use std::fs::File;

pub fn csv_reader(path: String) -> Reader<File> {
    csv::ReaderBuilder::new()
        .trim(Trim::All)
        .from_path(path)
        .unwrap()
}
