use std::path::PathBuf;

use clap::Parser;

use crate::answer::to_poll_answers;
use crate::maildata::get_mail_data;
use crate::question::load_poll_questions;
use crate::stats::PollStatistics;

mod answer;
mod maildata;
mod question;
mod stats;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'p', long)]
    poll: PathBuf,

    #[arg(short = 'f', long)]
    statsfile: PathBuf,

    #[arg(short = 'c', long)]
    cachedir: PathBuf,
}

fn main() {
    let args = Args::parse();
    let (qorder, poll) = load_poll_questions(&args.poll);
    let (id, data) = get_mail_data();

    let mut stats = PollStatistics::load(&args.statsfile);
    let answers = to_poll_answers(&poll, data);

    answer::save_in_cache(&id, &args.cachedir, &answers);

    for qslug in qorder {
        let question = poll.get(&qslug).unwrap();
        let Some(answer) = answers.get(&qslug) else {
            println!("> {}\nNo answer\n", question.text);
            continue;
        };
        println!("> {}\n \"{}\"\n", question.text, answer.display(question));
    }

    stats.feed(id, &poll, answers);
    stats.save(&args.statsfile);
}
