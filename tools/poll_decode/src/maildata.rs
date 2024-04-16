use std::collections::HashMap;

use base64::Engine;
use sha2::{Digest, Sha256};

#[path = "/home/john/work/web/ecoweb/tools/decode_mailed_data.rs"]
mod tool;

fn get_data_hash(data: &String) -> String {
    let mut hasher = Sha256::default();
    hasher.update(data);
    base64::engine::general_purpose::STANDARD.encode(hasher.finalize())
}

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
