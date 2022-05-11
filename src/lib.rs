use std::env;

fn get_yes_str() -> String {
    let mut args: Vec<String> = env::args().collect();

    return if args.len() > 1 {
        args.remove(0);
        args.join(" ")
    } else {
        "y".to_string()
    };
}

/// Get memory page-aligned buffer
pub fn get_buf() -> Vec<u8> {
    const BUF_SIZE: usize = 4096 * 2;

    let mut str = get_yes_str();
    str.push_str("\n");
    str.repeat(BUF_SIZE).into_bytes()
}
