use std::{path::PathBuf, collections::BTreeMap};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::Settings;
use crate::answer::PollAnswer;
use crate::load::PollQuestion;

#[derive(Debug, Serialize, Deserialize)]
pub enum PollQuestionStat {
    YesOrNo(usize, usize),     // How many Yes / No
    Radio(Vec<usize>),         // How many for each choice
    Checkbox(Vec<Vec<usize>>), // How many for each choice
    Number(BTreeMap<usize, usize>),    // All the numbers given
    Range(BTreeMap<usize, usize>),     // All the numbers given
    Text(Vec<String>),                 // All the texts given
}

impl PollQuestionStat {
    pub fn feed(&mut self, answer: PollAnswer) {
        todo!();
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PollStatistics {
    answers: usize,

    questions_stats: HashMap<PollQuestion, PollQuestionStat>,

    // Keep track of data already included in this stats
    accounted: Vec<String>,
}

impl PollStatistics {
    pub fn save(&self, fpath: &PathBuf) {
        let data = serde_json::to_string(&self).unwrap();
        std::fs::write(fpath, data).unwrap();
    }

    pub fn load(fpath: &PathBuf) -> PollStatistics {
        if !fpath.exists() {
            return PollStatistics::default();
        }

        let data = std::fs::read_to_string(fpath).unwrap();
        serde_json::from_str(&data).unwrap()
    }

    // Add the data to the statistics
    pub fn feed(&mut self, id: String, data: HashMap<PollQuestion, PollAnswer>) {
        for (question, answer) in data {
            if let Some(s) = self.questions_stats.get_mut(&question) {
                s.feed(answer)
            } else {
                self.questions_stats.insert(question, answer.into());
            }
        }
        self.accounted.push(id);
    }

    // Remove noise, update stats to a cleaner state, based on params
    pub fn clean(&mut self, params: &Settings) {
        // TODO    For every choice questions, merge the aliased answers
    }
}
