use args::ArgType;
use search::find_runner_in_dir;
use std::{env, process::ExitCode};

mod args;
mod runners;
mod search;

fn main() -> Result<ExitCode, std::io::Error> {
    let current = env::current_dir()?;
    let runner = find_runner_in_dir(&current)?;

    if let Some(runner) = runner {
        let args = collect_args();

        let mut last_command_i: Option<usize> = None;
        let mut subarg_stack = Vec::new();
        let mut is_quiet = false;

        for (i, arg) in args.iter().enumerate() {
            match ArgType::from(arg.as_str()) {
                ArgType::Flag => {
                    if last_command_i.is_some() {
                        subarg_stack.push(arg.as_str())
                    } else {
                        match arg.as_str() {
                            "--version" | "-V" => {
                                println!("{}", env!("CARGO_PKG_VERSION"));
                                return Ok(ExitCode::SUCCESS);
                            }
                            "--help" | "-h" => {
                                println!("{}", include_str!("../target/help_message.txt"));
                                return Ok(ExitCode::SUCCESS);
                            }
                            "--quiet" | "-q" => is_quiet = true,
                            _ => {}
                        }
                    }
                }
                ArgType::Subargs => {
                    subarg_stack.extend(arg.split_ascii_whitespace().collect::<Vec<_>>())
                }
                ArgType::None => {
                    if let Some(last_i) = last_command_i {
                        let command = &args[last_i];

                        let status = runner.run(command, &subarg_stack, is_quiet)?;
                        subarg_stack.clear();

                        if !status.success() {
                            return Ok(ExitCode::FAILURE);
                        }
                    }

                    last_command_i = Some(i);
                }
            }
        }

        if args.len() == 1 || last_command_i.is_none() {
            let status = runner.run("dev", &[], is_quiet)?;
            return Ok(match status.success() {
                true => ExitCode::SUCCESS,
                false => ExitCode::FAILURE,
            });
        }

        return Ok(ExitCode::SUCCESS);
    }

    eprintln!("No script runner config found in current directory.");
    Ok(ExitCode::FAILURE)
}

/// Collects arguments from std::env,
/// adds empty string to the end
fn collect_args() -> Vec<String> {
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    args.push("".into());

    args
}
