use std::io::{BufRead, Write};

use clap::Parser;

/// Transforms Android logcat from stdin to use relative timestamps.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Whether to flush the output after each line.
    #[clap(short, long)]
    line_buffered: bool,
}

fn main() {
    let args = Args::parse();

    let mut first_time: Option<chrono::NaiveDateTime> = None;
    let stdin = std::io::stdin();

    let lines = stdin.lock().lines().filter_map(|x| x.ok());
    for line in lines {
        let maybe_msg = logcat::parse::threadtime(&line);
        if let Err(e) = maybe_msg {
            println!("error: {}: {}", e, line);
            continue;
        }

        let msg = maybe_msg.unwrap();
        if first_time.is_none() && msg.date_time().is_some() {
            first_time = msg.date_time();
        }

        if let Some(date_time) = msg.date_time() {
            let duration = date_time.signed_duration_since(first_time.unwrap());
            let elapsed_secs = duration.num_milliseconds() as f64 / 1000.0;
            println!(
                "{:<10.3} {:<6} {:5} {} {:<8}: {}",
                elapsed_secs,
                msg.process_id().unwrap(),
                msg.thread_id().unwrap(),
                msg.level().short(),
                msg.tag(),
                msg.content()
            );
            if args.line_buffered {
                std::io::stdout()
                    .flush()
                    .expect("error: unable to flush stdout");
            }
        }
    }
}
