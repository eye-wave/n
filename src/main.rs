use search::find_runner_in_dir;
use std::{env, sync::LazyLock};

mod error;
mod runners;
mod search;

pub use error::{Error, Result};

static HELP_MESSAGE: LazyLock<String> =
    LazyLock::new(|| include_str!("./help.txt").replace("\\033", "\u{001B}"));

fn main() -> Result<()> {
    let current = env::current_dir()?;
    let runner = find_runner_in_dir(&current)?;

    if let Some(runner) = runner {
        let args = env::args().skip(1).collect::<Vec<_>>();

        for arg in args.iter() {
            match arg.as_str() {
                "--version" | "-V" => {
                    println!("{}", env!("CARGO_PKG_VERSION"));
                    return Ok(());
                }
                "--help" | "-h" => {
                    println!("{}", *HELP_MESSAGE);
                    return Ok(());
                }
                _ => runner.run(arg)?,
            }
        }

        if args.is_empty() {
            runner.run("dev")?
        }

        return Ok(());
    }

    eprintln!("No script runner config found in current directory.");
    Ok(())
}
