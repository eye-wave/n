use crate::runners::Runner;
use std::fs::{self, read_dir};
use std::path::Path;

pub fn find_runner_in_dir<P: AsRef<Path>>(path: &P) -> Result<Option<Runner>, std::io::Error> {
    let contents = read_dir(path.as_ref())?;
    let mut for_later = Vec::new();

    for file in contents.flatten() {
        match file.file_name().to_ascii_lowercase().to_str() {
            Some("package-lock.json") => return Ok(Some(Runner::Npm)),
            Some("yarn.lock") => return Ok(Some(Runner::Yarn)),
            Some("pnpm-lock.yaml") => return Ok(Some(Runner::Pnpm)),
            Some("bun.lockb") => return Ok(Some(Runner::Bun)),
            Some("bunfig.toml") => return Ok(Some(Runner::Bun)),
            Some("deno.json") => return Ok(Some(Runner::Deno)),
            Some("deno.lock") => return Ok(Some(Runner::Deno)),
            Some("xtask") => match fs::metadata(file.path()) {
                Ok(metadata) => {
                    if !metadata.is_dir() {
                        continue;
                    }

                    if !file.path().join("Cargo.toml").exists() {
                        continue;
                    }

                    return Ok(Some(Runner::Xtask));
                }
                _ => continue,
            },
            Some("makefile") => return Ok(Some(Runner::Makefile)),
            Some("justfile") => return Ok(Some(Runner::Justfile)),

            Some("cargo.toml") | Some("package.json") => for_later.push(file),

            Some(_) => continue,
            None => continue,
        }
    }

    for file in for_later {
        match file.file_name().to_ascii_lowercase().to_str() {
            Some("cargo.toml") => return Ok(Some(Runner::Cargo)),
            Some("package.json") => return Ok(Some(Runner::Npm)),
            _ => continue,
        }
    }

    Ok(None)
}
