use std::env;

#[derive(Debug)]
pub enum ArgType {
    Flag,
    Subargs,
    Command,
}

impl From<&str> for ArgType {
    fn from(value: &str) -> Self {
        let mut is_flag = false;

        for c in value.chars() {
            if c.is_whitespace() {
                return Self::Subargs;
            }

            if c == '-' {
                is_flag = true
            }
        }

        match is_flag {
            true => Self::Flag,
            false => Self::Command,
        }
    }
}

/// Collects arguments from std::env,
/// adds empty string to the end
pub fn collect_args() -> Vec<String> {
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    args.push("".into());

    args
}

pub fn split_into_subargs(arg: &str) -> Vec<&str> {
    arg.split_ascii_whitespace().collect()
}
