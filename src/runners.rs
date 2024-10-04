use crate::Result;
use xshell::{cmd, Shell};

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

impl Runner {
    fn to_string(&self) -> Option<&'static str> {
        match self {
            Self::Npm => Some("npm"),
            Self::Yarn => Some("yarn"),
            Self::Pnpm => Some("pnpm"),
            Self::Bun => Some("bun"),
            Self::Deno => Some("deno"),

            // implemented for javascript runner only
            // becuase this function is usefull just for them
            _ => None,
        }
    }
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

impl Runner {
    const JAVASCRIPT_NO_RUN_WITH: [&'static str; 2] = ["install", "update"];

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
            "i" => "install",
            "u" => "update",
            "d" => "dev",
            "f" => "format",
            "l" => "lint",
            "b" => "build",
            "t" => "test",
            "p" => "preview",
            "s" => "start",
            _ => command,
        }
    }

    pub fn run(&self, command: &str) -> Result<()> {
        let command = self.unalias_command(command);
        let sh = Shell::new()?;

        if let Language::Javascript = self.into() {
            if Self::JAVASCRIPT_NO_RUN_WITH.contains(&command) {
                let r = self.to_string().unwrap();
                cmd!(sh, "{r} {command}").run()?;

                return Ok(());
            }
        }

        match self {
            Self::Npm | Self::Yarn | Self::Pnpm | Self::Bun => {
                let r = self.to_string().unwrap();
                cmd!(sh, "{r} run {command}")
            }
            Self::Deno => cmd!(sh, "deno task {command}"),
            Self::Xtask => cmd!(sh, "cargo run --package xtask -- {command}"),
            Self::Cargo => cmd!(sh, "cargo {command}"),
            Self::Makefile => cmd!(sh, "make {command}"),
            Self::Justfile => cmd!(sh, "just {command}"),
        }
        .run()?;

        Ok(())
    }
}
