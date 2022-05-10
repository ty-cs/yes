use std::io;
use std::io::Write;

fn main() {
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());
    let yes_bytes = b"y\n";
    loop {
        writer.write(yes_bytes).unwrap();
    }
}
