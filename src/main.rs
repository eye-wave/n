use args::*;
use runners::bool_to_exit;
use search::create_scripts_map;
use std::{env, process::ExitCode};

mod args;
mod hashmap;
mod parsers;
mod runners;
mod search;

enum IterResult {
    GlobalFlag,
    SubargFlag,
    Subargs,
    Command(usize),
    Empty,
}

macro_rules! print_exit {
    ($msg:expr) => {{
        println!("{}", $msg);
        return Ok(ExitCode::SUCCESS);
    }};
}

macro_rules! print_fail {
    ($msg:expr) => {{
        eprintln!("{}", $msg);
        return Ok(ExitCode::FAILURE);
    }};
}

fn main() -> Result<ExitCode, std::io::Error> {
    let current = env::current_dir()?;
    let script_map = create_scripts_map(&current)?;

    if script_map.no_runners() {
        print_fail!("No script runner config found in current directory.");
    }

    let args = collect_args();

    let mut last_command_i: Option<usize> = None;
    let mut subarg_stack = Vec::new();
    let mut is_quiet = false;

    for (i, arg) in args.iter().enumerate() {
        let arg_type = ArgType::from(arg.as_str());
        let iter_result: IterResult = match arg_type {
            ArgType::Flag => match last_command_i {
                Some(_) => IterResult::SubargFlag,
                None => IterResult::GlobalFlag,
            },
            ArgType::Subargs => IterResult::Subargs,
            ArgType::Command => match last_command_i {
                Some(i) => IterResult::Command(i),
                None => IterResult::Empty,
            },
        };

        match iter_result {
            IterResult::GlobalFlag => match arg.as_str() {
                "--version" | "-V" => print_exit!(env!("CARGO_PKG_VERSION")),
                "--help" | "-h" => print_exit!(include_str!("../target/help_message.txt")),
                "--list" | "-l" => print_exit!(script_map.display()),
                "--list-all" | "-L" => print_exit!(script_map.display_all()),
                "--quiet" | "-q" => is_quiet = true,
                _ => eprintln!(r#"Received invalid flag "{}""#, arg),
            },
            IterResult::SubargFlag => subarg_stack.push(arg.as_str()),
            IterResult::Subargs => subarg_stack.extend(split_into_subargs(arg)),
            IterResult::Command(i) => {
                let command = &args[i];

                if let Some(runner) = script_map.find_runner(command) {
                    let status = runner.run(command, &subarg_stack, is_quiet)?;
                    subarg_stack.clear();

                    if !status.success() {
                        return Ok(ExitCode::FAILURE);
                    }
                }
            }
            _ => {}
        }

        if let ArgType::Command = arg_type {
            last_command_i = Some(i);
        }
    }

    if args.len() == 1 || last_command_i.is_none() {
        if let Some(runner) = script_map.find_runner("dev") {
            let status = runner.run("dev", &[], is_quiet)?;
            return Ok(bool_to_exit(status.success()));
        }
    }

    Ok(ExitCode::SUCCESS)
}
