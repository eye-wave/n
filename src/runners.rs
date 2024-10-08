use core::str;
use std::process::{Command, ExitCode, ExitStatus};

pub fn bool_to_exit(status: bool) -> ExitCode {
    match status {
        true => ExitCode::SUCCESS,
        false => ExitCode::FAILURE,
    }
}

macro_rules! include_vec {
    ($path:expr) => {{
        const CONTENTS: &str = include_str!($path);
        CONTENTS
            .split_ascii_whitespace()
            .filter(|s| !s.is_empty())
            .collect::<Vec<&'static str>>()
    }};
}

#[derive(PartialEq, Debug, Clone)]
pub enum Runner {
    // javascript
    Npm,
    Yarn,
    Pnpm,
    Bun,
    Deno,

    // rust
    Xtask,
    Cargo,

    // other
    Makefile,
}

pub enum Language {
    Javascript,
    Rust,
    Other,
}

impl From<&Runner> for Language {
    fn from(value: &Runner) -> Self {
        match value {
            Runner::Npm | Runner::Yarn | Runner::Pnpm | Runner::Bun | Runner::Deno => {
                Self::Javascript
            }
            Runner::Xtask | Runner::Cargo => Self::Rust,
            Runner::Makefile => Self::Other,
        }
    }
}

impl From<&Runner> for &str {
    fn from(runner: &Runner) -> &'static str {
        match runner {
            Runner::Npm => "npm",
            Runner::Yarn => "yarn",
            Runner::Pnpm => "pnpm",
            Runner::Bun => "bun",
            Runner::Deno => "deno",
            Runner::Cargo => "cargo",
            Runner::Xtask => "xtask",
            Runner::Makefile => "make",
        }
    }
}

fn spawn(command: &str, args: &[&str]) -> Result<ExitStatus, std::io::Error> {
    Command::new(command).args(args).spawn()?.wait()
}

impl From<Runner> for usize {
    fn from(value: Runner) -> usize {
        match value {
            Runner::Npm => 0,
            Runner::Yarn => 1,
            Runner::Pnpm => 2,
            Runner::Bun => 3,
            Runner::Deno => 4,
            Runner::Makefile => 5,
            Runner::Xtask => 6,
            Runner::Cargo => 7,
        }
    }
}

impl Runner {
    const JAVASCRIPT_SPECIAL_COMMANDS: &[&str] = &["install", "update", "add", "remove"];

    pub const NUMBER_OF_RUNNERS: usize = 8;

    pub(super) fn from_usize(value: usize) -> Option<Self> {
        match value {
            0 => Some(Runner::Npm),
            1 => Some(Runner::Yarn),
            2 => Some(Runner::Pnpm),
            3 => Some(Runner::Bun),
            4 => Some(Runner::Deno),
            5 => Some(Runner::Makefile),
            6 => Some(Runner::Xtask),
            7 => Some(Runner::Cargo),
            _ => None,
        }
    }

    fn unalias_cargo<'a>(&self, command: &'a str) -> &'a str {
        if let Self::Cargo = *self {
            match command {
                "d" | "dev" => "run",
                "f" | "format" => "fmt",
                "l" | "lint" => "clippy",
                _ => command,
            }
        } else {
            command
        }
    }

    pub fn unalias_command(command: &str) -> &str {
        match command {
            "a" => "add",
            "b" => "build",
            "d" => "dev",
            "f" => "format",
            "i" => "install",
            "l" => "lint",
            "p" => "preview",
            "s" => "start",
            "t" => "test",
            "u" => "update",
            _ => command,
        }
    }

    pub fn run(
        &self,
        command: &str,
        args: &[&str],
        quiet: bool,
    ) -> Result<ExitStatus, std::io::Error> {
        let command = &Self::unalias_command(command);
        let command = &self.unalias_cargo(command);

        let mut subargs = Vec::new();
        let runner_name: &str = self.into();
        let runner_name = if *self == Self::Xtask {
            "cargo"
        } else {
            runner_name
        };

        match self.into() {
            Language::Javascript => {
                if !Self::JAVASCRIPT_SPECIAL_COMMANDS.contains(command) {
                    let run = if *self == Self::Deno { "task" } else { "run" };

                    subargs.push(run);
                }
            }
            _ => {
                if *self == Self::Xtask {
                    subargs.extend(["run", "--package", "xtask", "--"]);
                }
            }
        }

        subargs.push(command);
        subargs.extend(args);

        if !quiet {
            let subargs = subargs.join(" ");
            println!("$ {runner_name} {subargs}");
        }

        spawn(runner_name, &subargs)
    }

    pub fn default_commands(&self) -> Vec<&'static str> {
        match self {
            Runner::Npm => include_vec!("./default/commands_npm.txt"),
            Runner::Yarn => include_vec!("./default/commands_yarn.txt"),
            Runner::Pnpm => include_vec!("./default/commands_pnpm.txt"),
            Runner::Bun => include_vec!("./default/commands_bun.txt"),
            Runner::Deno => include_vec!("./default/commands_deno.txt"),
            Runner::Cargo => include_vec!("./default/commands_cargo.txt"),
            Runner::Xtask => vec![],
            Runner::Makefile => vec![],
        }
    }
}
