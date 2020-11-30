use anyhow::{Context, Result};
use indicatif::ProgressBar;
use std::{time::Duration, io::{self, Write}};
use structopt::StructOpt;
use std::thread;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
    // let pb = indicatif::ProgressBar::new(100);
    // for i in 0..100 {
    //     pb.println(format!("[+] finished #{}", i));
    //     pb.inc(1);
    // }
    // pb.finish_with_message("done");
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{:?}`", &args.path))?;
    println!("len: {}", content.lines().count());
    let pb = indicatif::ProgressBar::new(100);
    for (num , line) in content.lines().enumerate() {
        if line.contains(&args.pattern) {
            writeln!(handle, "{}", line)?; // add `?` if you care about errors here
        }
        pb.inc((1 / content.lines().count() as u64) *100);
        thread::sleep(Duration::from_millis(500));
    }
    pb.finish_with_message("done");
    handle.flush()?;
    // println!("file content: {}", content);
    Ok(())
}
