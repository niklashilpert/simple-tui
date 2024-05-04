use std::io::{self, Write};


pub fn flush() -> io::Result<()> {
    io::stdout().flush()
}

pub fn await_input() {
    let mut buf = String::new();
    if let Err(e) = io::stdin().read_line(&mut buf) {
        println!("{}", e.to_string());
    }
}
