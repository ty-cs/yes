use std::io;
use std::io::Write;
use yes::get_buf;

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let buf = get_buf();
    loop {
        stdout.write(&buf)?;
    }
}
