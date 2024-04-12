use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{load::PollQuestion, stats::PollQuestionStat};


#[derive(Debug, Serialize, Deserialize)]
pub enum PollAnswer {
    Numeric(f64), // Average
    Choices(Vec<String>), // Unique choices in lowercase
}


impl PollAnswer {
    pub fn create(question: &PollQuestion, value: String) -> PollAnswer {
        todo!();
    }
}

impl Into<PollQuestionStat> for PollAnswer {
    fn into(self) -> PollQuestionStat {
        todo!()
    }
}

pub fn to_poll_answers(poll: HashMap<String, PollQuestion>, data: HashMap<String, String>) -> HashMap<PollQuestion, PollAnswer> {
    let mut answers = HashMap::new();
    for (key, val) in data {
        if !key.starts_with("q-") {
            continue;
        }
        let q = poll.get(&key).unwrap().clone();
        let answer = PollAnswer::create(&q, val);
        answers.insert(q, answer);
    }
    answers
}
