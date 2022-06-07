use csv::{Reader, Trim, Writer};
use std::fs::File;

pub fn csv_reader(path: String) -> Reader<File> {
    csv::ReaderBuilder::new()
        .trim(Trim::All)
        .from_path(path)
        .unwrap()
}

pub fn stdout_writer() -> Writer<std::io::Stdout> {
    let w = std::io::stdout();
    csv::WriterBuilder::new().from_writer(w)
}
