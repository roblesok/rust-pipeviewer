use std::env;
use std::io::{self, ErrorKind, Read, Result, Write};

const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> Result<()> {
    let mut total_bytes = 0;
    let mut buffer = [0; CHUNK_SIZE];
    let silent = !env::var("PV_SILENT").unwrap_or_default().is_empty();
    loop {
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        total_bytes += num_read;
        if !silent {
            eprint!("\rRead: {}", total_bytes);
        }
        if let Err(e) = io::stdout().write_all(&buffer[..num_read]) {
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }
            // return Err(e)
            eprintln!("Oh no, an error ocurred: {}", e.to_string());
            std::process::exit(1);
        }
    }
    if !silent {
        eprintln!("\rRead: {}", total_bytes)
    }
    Ok(())
}
