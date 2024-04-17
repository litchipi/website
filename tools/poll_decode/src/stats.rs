use std::{path::PathBuf, collections::BTreeMap};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::answer::PollAnswer;
use crate::question::PollQuestion;

#[derive(Debug, Serialize, Deserialize)]
pub enum PollQuestionStat {
    YesOrNo(usize, usize),     // How many Yes / No
    Radio(Vec<usize>),         // How many for each choice
    Checkbox(Vec<usize>), // How many for each choice
    Number(BTreeMap<usize, usize>),    // All the numbers given
    Range(BTreeMap<usize, usize>),     // All the numbers given
    Text(Vec<String>),                 // All the texts given
}

impl PollQuestionStat {
    pub fn feed(&mut self, answer: PollAnswer) {
        match (self, answer) {
            (PollQuestionStat::YesOrNo(ref mut nyes, ref mut nno),
                PollAnswer::YesOrNo(res)
            ) => if res {
                *nyes += 1;
            } else {
                *nno += 1;
            },

            (PollQuestionStat::Radio(ref mut choices), PollAnswer::Radio(choice)) => {
                *choices.get_mut(choice).unwrap() += 1;
            }

            (PollQuestionStat::Checkbox(ref mut stats), PollAnswer::Checkbox(checked)) => {
                checked.iter().for_each(|c| *stats.get_mut(*c).unwrap() += 1);
            }

            (PollQuestionStat::Number(ref mut map), PollAnswer::Numeric(nb))
            | (PollQuestionStat::Range(ref mut map), PollAnswer::Range(nb)) => {
                if let Some(count) = map.get_mut(&nb) {
                    *count += 1;
                } else {
                    map.insert(nb, 1);
                }
            }

            (PollQuestionStat::Text(ref mut textes), PollAnswer::Text(t)) => {
                textes.push(t);
            }

            (s, a) => panic!("Feeding incompatible answer {a:?} to stat {s:?}"),
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PollStatistics {
    answers: usize,

    questions_stats: HashMap<String, PollQuestionStat>,

    // Keep track of data already included in this stats
    accounted: Vec<String>,
}

impl PollStatistics {
    pub fn save(&self, fpath: &PathBuf) {
        let data = serde_json::to_string(self).unwrap();
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
    pub fn feed(&mut self, id: String, poll: &HashMap<String, PollQuestion>, data: HashMap<String, PollAnswer>) {
        if self.accounted.contains(&id) {
            println!("Answer already accounted for in stats");
            return;
        }
        for (slug, answer) in data {
            if let Some(s) = self.questions_stats.get_mut(&slug) {
                s.feed(answer)
            } else {
                let stat = answer.init_stat(poll.get(&slug).unwrap());
                self.questions_stats.insert(slug.clone(), stat);
            }
        }
        self.accounted.push(id);
    }
}
