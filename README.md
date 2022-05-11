# yes 

[![](https://github.com/ty-cs/yes/actions/workflows/CI.yml/badge.svg)](https://github.com/ty-cs/yes/actions/workflows/CI.yml)

GNU yes in Rust.

## Performance

Performance is measured by executing the following command on a Debian server:
```bash
yes | pv > /dev/null
```

- GNU version: `~2.5GiB/s`
- Rust version: `~3.3GiB/s`
