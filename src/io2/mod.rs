use std::io::stdin;

pub fn read_line() -> Option<String> {
    let terminal = stdin();
    let mut output = String::new();

    match terminal.read_line(&mut output) {
        Ok(_) => Some(output),
        Err(_) => None,
    }
}

pub fn read_i64() -> Option<i64> {
    let line = read_line()?;
    match line.trim().parse() {
        Ok(i) => Some(i),
        Err(_) => None,
    }
}
