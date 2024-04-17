use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

use crate::question::PollQuestion;
use crate::stats::PollQuestionStat;

#[derive(Debug, Serialize, Deserialize)]
pub enum PollAnswer {
    Numeric(usize),
    Radio(usize),
    Range(usize),
    YesOrNo(bool),
    Text(String),
    Checkbox(Vec<usize>),
}

impl PollAnswer {
    pub fn create(slug: String, question: &PollQuestion, value: String) -> PollAnswer {
        match question.qtype.as_str() {
            "yes_or_no" => PollAnswer::YesOrNo(value == "true"),
            "number" => PollAnswer::Numeric(
                value
                    .parse()
                    .unwrap_or_else(|_| panic!("Unable to parse question {slug} number value")),
            ),
            "checkbox" => {
                let n = value
                    .parse()
                    .unwrap_or_else(|_| panic!("Unable to parse question {slug} checkbox value"));
                PollAnswer::Checkbox(vec![n])
            }
            "range" => PollAnswer::Range(
                value
                    .parse()
                    .unwrap_or_else(|_| panic!("Unable to parse question {slug} range value")),
            ),
            "radio" => PollAnswer::Radio(
                value
                    .parse()
                    .unwrap_or_else(|_| panic!("Unable to parse question {slug} radio value")),
            ),
            "text" => PollAnswer::Text(value),
            _ => unimplemented!("Question type {}", question.qtype),
        }
    }

    pub fn merge_with(&mut self, other: PollAnswer) {
        match (self, other) {
            (PollAnswer::Checkbox(na), PollAnswer::Checkbox(nb)) => {
                nb.iter().for_each(|n| {
                    if !na.contains(n) {
                        na.push(*n)
                    }
                });
            }
            (a, b) => unimplemented!("Attempt to merge answer {:?} with {:?}", a, b),
        }
    }

    pub fn init_stat(self, question: &PollQuestion) -> PollQuestionStat {
        match self {
            PollAnswer::Numeric(nb) => {
                let mut map = BTreeMap::new();
                map.insert(nb, 1);
                PollQuestionStat::Number(map)
            }
            PollAnswer::Radio(choice) => {
                let nb_choices = question.choices.as_ref().unwrap().len();
                let mut v = vec![0; nb_choices];
                *v.get_mut(choice).unwrap() = 1;
                PollQuestionStat::Radio(v)
            }
            PollAnswer::Range(nb) => {
                let mut map = BTreeMap::new();
                map.insert(nb, 1);
                PollQuestionStat::Range(map)
            }
            PollAnswer::YesOrNo(ans) => {
                let y = if ans { 1 } else { 0 };
                let n = if ans { 0 } else { 1 };
                PollQuestionStat::YesOrNo(y, n)
            }
            PollAnswer::Text(t) => PollQuestionStat::Text(vec![t]),
            PollAnswer::Checkbox(choices) => {
                let nb_choices = question.choices.as_ref().unwrap().len();
                let mut v = vec![0; nb_choices];
                choices.iter().for_each(|c| *v.get_mut(*c).unwrap() = 1);
                PollQuestionStat::Checkbox(v)
            }
        }
    }
}

pub fn to_poll_answers(
    poll: &HashMap<String, PollQuestion>,
    data: HashMap<String, String>,
) -> HashMap<String, PollAnswer> {
    let mut answers: HashMap<String, PollAnswer> = HashMap::new();
    for (key, val) in data {
        if val.is_empty() {
            continue;
        }
        if !key.starts_with("q-") {
            continue;
        }

        let mut key_split = key.split('-');
        let Some(qset) = key_split.nth(1) else {
            println!("Unable to get qset from key {key}");
            continue;
        };
        let Some(qslug) = key_split.next() else {
            println!("Unable to get qslug from key {key}");
            continue;
        };
        let qkey = format!("q-{qset}-{qslug}");
        let Some(q) = poll.get(&qkey) else {
            println!("Key {key} not found in poll, skipping...");
            continue;
        };
        let q = q.clone();
        let answer = PollAnswer::create(key.clone(), &q, val);
        if let Some(asw) = answers.get_mut(&qkey) {
            asw.merge_with(answer);
        } else {
            answers.insert(qkey, answer);
        }
    }
    answers
}
