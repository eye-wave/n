use std::process::{Command, ExitStatus};

#[derive(PartialEq)]
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
    Justfile,
    Makefile,
}

enum Language {
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
            Runner::Justfile | Runner::Makefile => Self::Other,
        }
    }
}

fn spawn(command: &str, args: &[&str]) -> Result<ExitStatus, std::io::Error> {
    Command::new(command).args(args).spawn()?.wait()
}

impl Runner {
    const JAVASCRIPT_SPECIAL_COMMANDS: [&'static str; 4] = ["install", "update", "add", "remove"];

    fn unalias_command<'a>(&self, command: &'a str) -> &'a str {
        if let Self::Cargo = *self {
            match command {
                "d" | "dev" => return "run",
                "f" | "format" => return "fmt",
                "l" | "lint" => return "clippy",
                _ => {}
            }
        }

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
        let command = self.unalias_command(command);

        let mut subargs = Vec::new();
        let runner_name = match self {
            Self::Npm => "npm",
            Self::Yarn => "yarn",
            Self::Pnpm => "pnpm",
            Self::Bun => "bun",
            Self::Deno => "deno",
            Self::Cargo | Self::Xtask => "cargo",
            Self::Justfile => "just",
            Self::Makefile => "make",
        };

        match self.into() {
            Language::Javascript => {
                if !Self::JAVASCRIPT_SPECIAL_COMMANDS.contains(&command) {
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
}
