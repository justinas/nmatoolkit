use std::error::Error;
use std::io;
use std::io::stdin;

pub fn read_line() -> Result<String, io::Error> {
    let terminal = stdin();
    let mut output = String::new();
    terminal.read_line(&mut output)?;
    Ok(output)
}

pub fn read_i64() -> Result<i64, Box<dyn Error>> {
    let line = read_line()?;
    let number: i64 = line.trim().parse()?;
    Ok(number)
}
