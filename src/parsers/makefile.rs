use std::{fs, path::Path};

pub fn parse_makefile_targets<P: AsRef<Path>>(path: &P) -> Result<Vec<String>, std::io::Error> {
    let content = fs::read_to_string(path)?;
    let mut targets = Vec::new();
    let mut buffer = Vec::new();
    let mut is_line_ignored = false;

    for c in content.chars() {
        if is_line_ignored {
            if c == '\n' {
                is_line_ignored = false;
            }
            continue;
        }

        if c.is_alphabetic() {
            buffer.push(c);
        } else if c == ':' && !buffer.is_empty() {
            targets.push(buffer.iter().collect::<String>());
            buffer.clear();
            is_line_ignored = true;
        } else if c == ' ' && buffer.is_empty() {
            continue;
        } else if !buffer.is_empty() {
            buffer.clear();
            is_line_ignored = true;
        }
    }

    Ok(targets)
}
