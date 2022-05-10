use std::io;
use std::io::Write;

fn main() -> io::Result<()> {
    // let stdout = io::stdout();
    // let stdout = stdout.lock();
    // let mut writer = io::BufWriter::new(stdout);
    // let yes_bytes = b"y\n";
    //
    // loop {
    //     writer.write(yes_bytes)?;
    // }

    let buffer = "y\n".repeat(4096).into_bytes();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    loop {
        stdout.write(&buffer).unwrap();
    }
}
