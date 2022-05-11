use std::env;
pub fn get_buf() -> Vec<u8> {
    // pages-aligned buffer
    const BUF_SIZE: usize = 4096 * 2;
    let mut args: Vec<String> = env::args().collect();
    return if args.len() > 1 {
        args.remove(0);
        let mut str = args.join(" ");
        str.push_str("\n");
        str.repeat(BUF_SIZE).into_bytes()
    } else {
        "y\n".repeat(BUF_SIZE).into_bytes()
    };
}
