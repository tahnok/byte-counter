use std::env;
use std::ascii;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::time::Instant;

const BLOCK_SIZE: usize = 1024 * 32;

fn main() -> std::io::Result<()> {
    let mut args = env::args();
    if args.len() != 2 {
        return Err(Error::other("Pass one file path as arg to program"));
    }
    let path = args.nth(1).unwrap();
    println!("Opening {path}");

    let mut f = File::open(path)?;

    let mut byte_counts: Vec<u64> = vec![0; 256];

    let mut buffer = [0; BLOCK_SIZE];
    let mut num_bytes = 0usize;

    let t1 = Instant::now();

    loop {
        let n = f.read(&mut buffer[..])?;
        if n == 0 {
            break;
        }
        for i in 0..n {
            num_bytes += 1;
            let b = buffer[i] as usize;
            byte_counts[b] += 1;
        }
    }
    let dur = t1.elapsed().as_secs_f64();
    let num_bytes = (num_bytes as f64) / 1_048_576f64;
    println!("Read speed: {:.1} MiB / s", num_bytes / dur);

      for (key, val) in byte_counts.iter().enumerate() {
        let ascii = ascii::escape_default(key as u8).to_string();
        println!("{val},{key},{ascii}");
    }
    Ok(())
}
