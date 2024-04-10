#[path = "/home/john/work/web/ecoweb/tools/decode_mailed_data.rs"]
mod tool;

fn main() {
    let result = tool::decode_mailed_data("./data".into(), "./privk".into());
    println!("Name: {}", result["name"]);
    println!("Email: {}", result["email"]);
    println!("Subject: {}", result["subject"]);
    println!("Body:\n{}\n", result["body"]);
}
