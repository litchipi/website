use std::collections::HashMap;

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
    Checkbox(usize),
}


impl PollAnswer {
    pub fn create(slug: String, question: &PollQuestion, value: String) -> PollAnswer {
        // println!("{slug} {question:?} {value}");
        match question.qtype.as_str() {
            "yes_or_no" => PollAnswer::YesOrNo(value == "true"),
            "number" => PollAnswer::Numeric(value.parse()
                .expect(format!("Unable to parse question {slug} number value").as_str())
            ),
            // TODO    Get the original question slug from choice slug, and add it
            //        to enum data so that we can merge them into a single poll stat
            "checkbox" => PollAnswer::Checkbox(value.parse()
                .expect(format!("Unable to parse question {slug} checkbox value").as_str())
            ),
            "range" => PollAnswer::Range(value.parse()
                .expect(format!("Unable to parse question {slug} range value").as_str())
            ),
            "radio" => PollAnswer::Radio(value.parse()
                .expect(format!("Unable to parse question {slug} radio value").as_str())
            ),
            "text" => PollAnswer::Text(value),
            _ => unimplemented!("Question type {}", question.qtype),
            
        }
    }
}

impl Into<PollQuestionStat> for PollAnswer {
    fn into(self) -> PollQuestionStat {
        PollQuestionStat::YesOrNo(0, 0) // TODO Create stat from poll answer
    }
}

pub fn to_poll_answers(poll: HashMap<String, PollQuestion>, data: HashMap<String, String>) -> HashMap<String, PollAnswer> {
    let mut answers = HashMap::new();
    for (key, val) in data {
        if val.is_empty() {
            continue;
        }
        if !key.starts_with("q-") {
            continue;
        }
        let q = poll.get(&key).unwrap().clone();
        let answer = PollAnswer::create(key.clone(), &q, val);
        answers.insert(key, answer);
    }
    answers
}
