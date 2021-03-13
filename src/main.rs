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
    let mut curr_name = String::new();
    let mut counter = 1;

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

    wrt.write_record(&["#subject", "s_start", "s_end", "query", "occ_count"])?;
    wrt.flush()?;

    for result in rdr.deserialize() {
        let mut record: Record = result?;

        if curr_name == record.query {
            counter += 1;
        } else {
            curr_name = String::from(&record.query);
            counter = 1;
        }

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
            counter.to_string(),
        ])?;
        wrt.flush()?;
    }

    Ok(())
}
