use std::{collections::HashMap, path::PathBuf};

use base64::Engine;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use toml::map::Map;

#[derive(Clone, Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct PollQuestion {
    text: String,
    qtype: String,
    note: Option<String>,

    // Radio
    choices: Option<Vec<String>>,

    // Number
    min: Option<usize>,
    max: Option<usize>,

    // Yes or No
    addq_no: Option<String>,
    addq_yes: Option<String>,
}

impl From<&Map<String, toml::Value>> for PollQuestion {
    fn from(tmap: &toml::map::Map<String, toml::Value>) -> Self {
        // println!("\n{tmap:?}");
        PollQuestion {
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

fn get_data_hash(data: &String) -> String {
    let mut hasher = Sha256::default();
    hasher.update(data);
    base64::engine::general_purpose::STANDARD.encode(hasher.finalize())
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
            hmap.insert(qset_slug, PollQuestion::from(q.as_table().unwrap()));
        }
    }
    hmap
}

#[path = "/home/john/work/web/ecoweb/tools/decode_mailed_data.rs"]
mod tool;

pub fn get_mail_data() -> (String, HashMap<String, String>) {
    println!("Copy and paste the mail data over here: ");
    let mut data = String::new();
    loop {
        let n = std::io::stdin().read_line(&mut data).unwrap();
        if n == 1 {
            break;
        }
    }

    let data_id = get_data_hash(&data);
    (data_id, tool::decode_mailed_data(data, "./privk".into()))
}
