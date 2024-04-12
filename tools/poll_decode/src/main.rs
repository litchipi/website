use std::path::PathBuf;
use clap::Parser;

use crate::load::{get_mail_data, load_poll_questions};
use crate::stats::PollStatistics;

mod load;
mod stats;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    poll: PathBuf,

    #[arg(short, long)]
    statsfile: PathBuf,
}

fn main() {
    let args = Args::parse();
    let poll = load_poll_questions(&args.poll);
    let (id, data) = get_mail_data();

    let mut stats = PollStatistics::load(&args.statsfile);

    for (key, val) in data {
        if !key.starts_with("q-") {
            continue;
        }
        let q = poll.get(&key).unwrap();
        println!("{q:?}");
    }

    // save_to_history(result);
}
