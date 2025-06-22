
// Base dependencies
use std::io::{self, Write};


/*
    Function to print a buffer of characters
    to the console with specified width and height.
*/
pub fn print_buffer(buffer: &[char], width: usize, height: usize) -> io::Result<()> {
    let mut stdout = io::stdout();
    for k in 0..(width * height) {
        let c = if k % width != 0 {
            buffer[k]
        } else {
            '\n' // 10 is ASCII newline
        };
        print!("{}", c); // Or use `stdout.write_all(&[c as u8])?` for raw bytes
    }
    stdout.flush()?; // Ensure output is printed immediately
    Ok(())
}