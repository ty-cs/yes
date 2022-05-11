use std::io;
use std::io::Write;
use yes::get_buf;

fn main() -> io::Result<()> {
    // let stdout = io::stdout();
    // let stdout = stdout.lock();
    // let mut writer = io::BufWriter::new(stdout);
    // let yes_bytes = b"y\n";
    //
    // loop {
    //     writer.write(yes_bytes)?;
    // }

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let buf = get_buf();
    loop {
        stdout.write(&buf).unwrap();
    }
}
