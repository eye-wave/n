#[derive(Debug)]
pub enum ArgType {
    Flag,
    Subargs,
    None,
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
            false => Self::None,
        }
    }
}
