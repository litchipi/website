use std::collections::HashMap;
use std::path::PathBuf;

use clap::Parser;
use serde::Deserialize;

use crate::answer::to_poll_answers;
use crate::load::{get_mail_data, load_poll_questions};
use crate::stats::PollStatistics;

mod answer;
mod load;
mod stats;
mod handle;

#[derive(Debug, Deserialize)]
pub struct Settings {
    #[serde(default)]
    choices_aliases: HashMap<String, Vec<String>>, // Texts that outputs to a given choice
}

impl Settings {
    pub fn load(fpath: &PathBuf) -> Settings {
        let data = std::fs::read_to_string(fpath).unwrap();
        toml::from_str(&data).unwrap()
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    poll: PathBuf,

    #[arg(short, long)]
    statsfile: PathBuf,

    #[arg(short, long)]
    settings: PathBuf,
}

fn main() {
    let args = Args::parse();
    let poll = load_poll_questions(&args.poll);
    let params = Settings::load(&args.settings);
    let (id, data) = get_mail_data();

    let mut stats = PollStatistics::load(&args.statsfile);
    let answers = to_poll_answers(poll, data);
    stats.feed(id, answers);
    stats.clean(&params);
    stats.save(&args.statsfile);
}
