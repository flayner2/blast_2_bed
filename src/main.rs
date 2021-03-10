use serde::Deserialize;
use std::error::Error;
use std::io;

#[derive(Debug, Deserialize)]
struct Record {
    subject: String,
    start: u32,
    end: u32,
    query: String,
    count: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .double_quote(false)
        .flexible(true)
        .comment(Some(b'#'))
        .from_reader(io::stdin());

    for record in rdr.records() {}

    Ok(())
}
