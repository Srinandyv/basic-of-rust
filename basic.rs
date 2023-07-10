use std::fmt;
use urlencoding::encode;

struct Programmer {
    name: String,
    role: String,
    language_spoken: Vec<String>,
    code: Vec<String>,
    quote: String,
}
impl Programmer {
    fn say_hi(&self) {
        println!("Thanks for dropping by, hope you find some of my work interesting.");
    }
}
fn main() {
    let programmer = Programmer {
        name: "srinandy".to_string(),
        role: "Developer".to_string(),
        language_spoken: vec!["ta-IN".to_string(), "en_US".to_string()],
        code: vec!["C".to_string(), "Rust".to_string()],
        quote: "what we think we become.".to_string(),
    };
    let email = "srinandys@gmail.com";
    let link = format!(
        "mailto:{}",
        encode(email)
    );
    programmer.say_hi();
}

