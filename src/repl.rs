use std::io;
use std::io::Write;

use teal::interpreter::Walker;

pub fn start() -> io::Result<()> {
    let walker = Walker::new();
    let mut input = String::new();
    let mut count = 0_usize;

    loop {
        print!("[{}]: ", count);
        io::stdout().flush()?;

        input.clear();
        let len = io::stdin().read_line(&mut input)?;
        if len == 0 {
            // Reached EOF, erase control character and exit
            println!("{}", 8_u8 as char);
            return Ok(());
        }

        input = input.trim().to_owned();
        if input.is_empty() {
            continue;
        }

        walker.run(&input);
        count += 1;
    }
}
