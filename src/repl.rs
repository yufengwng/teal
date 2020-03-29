use std::io;
use std::io::Write;

use teal::interpreter::Walker;

pub fn start() -> io::Result<()> {
    let walker = Walker::new();
    let mut input = String::new();
    loop {
        print!("λ= ");
        io::stdout().flush()?;

        input.clear();
        let len = io::stdin().read_line(&mut input)?;
        if len == 0 {
            // Reached EOF, erase control character and exit
            println!("{}", 8_u8 as char);
            return Ok(());
        }
        if input.is_empty() {
            println!();
            continue;
        }

        walker.run(&input);
    }
}
