use serde::Deserialize;
use std::error::Error;
use std::io;

#[derive(Debug, Deserialize)]
struct Record {
    query: String,
    subject: String,
    expect: f64,
    q_start: usize,
    q_end: usize,
    s_start: usize,
    s_end: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .double_quote(false)
        .flexible(true)
        .comment(Some(b'#'))
        .from_reader(io::stdin());

    let mut wrt = csv::WriterBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .double_quote(false)
        .from_writer(io::stdout());

    for result in rdr.deserialize() {
        let mut record: Record = result?;

        if record.s_start > record.s_end {
            let temp = record.s_start;
            record.s_start = record.s_end;
            record.s_end = temp;
        }

        record.s_start -= 1;

        wrt.write_record(&[
            record.subject,
            record.s_start.to_string(),
            record.s_end.to_string(),
            record.query,
        ])?;
        wrt.flush()?;
    }

    Ok(())
}
