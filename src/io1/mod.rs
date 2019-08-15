//! Input/output utilities without proper error handling.
//! Functions return empty values on error.

use std::io::stdin;

pub fn read_line() -> String {
    let terminal = stdin();
    let mut output = String::new();

    match terminal.read_line(&mut output) {
        Ok(_) => (),
        Err(e) => println!("Error while reading a line: {}", e),
    }
    output
}

pub fn read_i64() -> i64 {
    let line = read_line();
    match line.trim().parse() {
        Ok(i) => i,
        Err(e) => {
            println!("Error while converting {:?} to an integer: {:?}", line, e);
            0
        }
    }
}
