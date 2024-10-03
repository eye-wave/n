use crate::Result;
use xshell::{cmd, Shell};

#[derive(Debug, PartialEq)]
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
    fn unalias_command<'a>(&self, command: &'a str) -> &'a str {
        if *self == Self::Cargo {
            match command {
                "d" | "dev" => return "run",
                "f" | "format" => return "fmt",
                "l" | "lint" => return "clippy",
                _ => {}
            }
        }

        match command {
            "d" => "dev",
            "f" => "format",
            "l" => "lint",
            "t" => "test",
            "p" => "preview",
            "s" => "start",
            _ => command,
        }
    }

    pub fn run(&self, command: &str) -> Result<()> {
        let command = self.unalias_command(command);
        let sh = Shell::new()?;

        match self {
            Self::Npm => cmd!(sh, "npm run {command}"),
            Self::Yarn => cmd!(sh, "yarn run {command}"),
            Self::Pnpm => cmd!(sh, "pnpm run {command}"),
            Self::Bun => cmd!(sh, "bun run {command}"),
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
