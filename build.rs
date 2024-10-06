use regex_lite::{Regex,Captures};
use std::fs;

fn main() -> Result<(), std::io::Error> {
    let file = fs::read_to_string("src/help.txt")?;
    let regex = Regex::new(r"(?m)\$\{A([0-9]+)\}").unwrap();
    let substitution = |caps: &Captures| {
        let color_code = &caps[1];
        format!("\x1b[{}m", color_code)
    };

    let result = regex.replace_all(&file, substitution);
        
    fs::write("target/help_message.txt", result.to_string())?;

    Ok(())
}
