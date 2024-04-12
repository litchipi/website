use std::path::PathBuf;
use std::collections::HashMap;
use serde_json::Value;

pub struct PollStatistics {
    answers: usize,
    numeric_or_not: (usize, usize),

    // TODO    Add statistics about questions

    // Keep track of data already included in this stats
    accounted: Vec<String>,
}

impl PollStatistics {
    fn empty() -> PollStatistics {
        PollStatistics {
        }
    }

    pub fn load(fpath: &PathBuf) -> PollStatistics {
        if !fpath.exists() {
            return PollStatistics::empty();
        }

        let data = std::fs::read_to_string(fpath).unwrap();
        let data : HashMap<String, Value> = serde_json::from_str(&data).unwrap();
        PollStatistics {
        }
    }
}
