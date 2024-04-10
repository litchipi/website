use std::collections::HashMap;

#[path = "/home/john/work/web/ecoweb/tools/decode_mailed_data.rs"]
mod tool;

fn save_to_history(data: HashMap<String, String>) {
    // TODO    Create a history directory, save the data inside for archives
}

fn main() {
    println!("Copy and paste the mail data over here: ");
    let mut data = String::new();
    loop {
        let n = std::io::stdin().read_line(&mut data).unwrap();
        if n == 1 {
            break;
        }
    }

    let result = tool::decode_mailed_data(data, "./privk".into());
    println!("Name: {}", result["name"]);
    println!("Email: {}", result["email"]);
    println!("Subject: {}", result["subject"]);
    println!("Body:\n{}\n", result["body"]);

    save_to_history(result);
}
