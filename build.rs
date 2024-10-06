use regex_lite::{Captures, Regex};
use std::fs;

const HELP_MESSAGE: &str = "${A4}Usage${A0}: ru [SCRIPT1] [SCRIPT2] ...

${A4}Default aliases${A0}:
  Cargo only:
    ${A1}dev${A0}, ${A1}d${A0}: run
    ${A1}format${A0}, ${A1}f${A0}: fmt
    ${A1}lint${A0}, ${A1}l${A0}: clippy

  ${A1}a${A0}: add
  ${A1}d${A0}: dev
  ${A1}f${A0}: format
  ${A1}i${A0}: install
  ${A1}l${A0}: lint
  ${A1}p${A0}: preview
  ${A1}s${A0}: start
  ${A1}t${A0}: test
  ${A1}u${A0}: update

${A4}Options${A0}:
  --help
    Display this help message
  
  --version
    Display current version
";

fn main() -> Result<(), std::io::Error> {
    let regex = Regex::new(r"(?m)\$\{A([0-9]+)\}").unwrap();
    let substitution = |caps: &Captures| {
        let color_code = &caps[1];
        format!("\x1b[{}m", color_code)
    };

    let result = regex.replace_all(HELP_MESSAGE, substitution);

    fs::write("target/help_message.txt", result.to_string())
}
