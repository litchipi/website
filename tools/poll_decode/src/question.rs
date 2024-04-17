use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use toml::map::Map;

#[derive(Clone, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct PollQuestion {
    #[serde(skip)]
    pub slug: String,
    
    text: String,
    pub qtype: String,
    note: Option<String>,

    // Radio
    pub choices: Option<Vec<String>>,

    // Number
    pub min: Option<usize>,
    pub max: Option<usize>,

    // Yes or No
    pub addq_no: Option<String>,
    pub addq_yes: Option<String>,
}

impl From<&Map<String, toml::Value>> for PollQuestion {
    fn from(tmap: &toml::map::Map<String, toml::Value>) -> Self {
        PollQuestion {
            slug: String::new(),
            text: get_toml_string("text", tmap).unwrap(),
            qtype: get_toml_string("type", tmap).unwrap(),
            note: get_toml_string("note", tmap),
            choices: tmap.
                get("choices")
                .map(|a| a
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|choice| choice.as_str().unwrap().to_string())
                    .collect()
                ),
            min: tmap.get("min").map(|n| n.as_integer().unwrap() as usize),
            max: tmap.get("max").map(|n| n.as_integer().unwrap() as usize),
            addq_no: get_toml_string("addq_no", tmap),
            addq_yes: get_toml_string("addq_yes", tmap),
        }
    }
}

fn get_toml_string(key: &str, tmap: &Map<String, toml::Value>) -> Option<String> {
    tmap.get(key).map(|v| v.as_str().unwrap().to_string())
}


pub fn load_poll_questions(fpath: &PathBuf) -> HashMap<String, PollQuestion> {
    let data = std::fs::read_to_string(fpath).unwrap();
    let data : HashMap<String, toml::Value> = toml::from_str(&data).unwrap();
    let questions = data.get("question").unwrap().as_table().unwrap();
    let mut hmap = HashMap::new();
    for (qset, qtable) in questions {
        let qtable = qtable.as_table().unwrap();
        for (qslug, q) in qtable {
            let qset_slug = format!("q-{qset}-{qslug}");
            let mut question = PollQuestion::from(q.as_table().unwrap());
            question.slug = qslug.clone();
            hmap.insert(qset_slug, question);
        }
    }
    hmap
}
