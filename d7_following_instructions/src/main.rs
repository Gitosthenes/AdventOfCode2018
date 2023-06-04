use std::{error, fs, result};

type Result<T> = result::Result<T, Box<dyn error::Error>>;
fn main() -> Result<()> {
    let input = fs::read_to_string("test_input.txt")?;

    println!("{}", &input);

    Ok(())
}
