use io::prelude::*;
use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let result = ics_filter::filter_from_query(&buffer);

    println!("{}", result);

    Ok(())
}
