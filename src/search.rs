use crate::hashmap::ScriptMap;
use crate::parsers::{parse_makefile_targets, parse_package_json_scripts};
use crate::runners::{Language, Runner};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

macro_rules! unwrap_or_continue {
    ($expr:expr) => {{
        if $expr.is_none() {
            continue;
        }
        $expr.unwrap()
    }};
}

pub fn create_scripts_map<P: AsRef<Path>>(path: &P) -> Result<ScriptMap, std::io::Error> {
    let mut current_level: PathBuf = path.as_ref().join("_");
    let mut script_map = ScriptMap::default();

    while let Some(parent) = current_level.parent() {
        current_level = parent.into();

        let walkdir = WalkDir::new(&*current_level).max_depth(1).into_iter();
        for entry in walkdir.filter_map(Result::ok) {
            let filename = entry.file_name().to_str();
            let filename = unwrap_or_continue!(filename);

            let runner = match filename.to_lowercase().as_str() {
                "package-lock.json" => Some(Runner::Npm),
                "yarn.lock" => Some(Runner::Yarn),
                "pnpm-lock.yaml" => Some(Runner::Pnpm),
                "bun.lockb" | "bunfig.toml" => Some(Runner::Bun),
                "deno.json" | "deno.lock" => Some(Runner::Deno),
                "xtask" => match fs::metadata(entry.path()) {
                    Ok(metadata) => {
                        if !metadata.is_dir() {
                            continue;
                        }
                        if !entry.path().join("Cargo.toml").exists() {
                            continue;
                        }

                        Some(Runner::Xtask)
                    }
                    _ => None,
                },
                "cargo.toml" | "cargo.lock" => Some(Runner::Cargo),
                "makefile" => {
                    if script_map.has_runner(Runner::Makefile) {
                        continue;
                    }

                    let path = current_level.join(filename);
                    if let Ok(scripts) = parse_makefile_targets(&path) {
                        script_map.add_scripts(&Runner::Makefile, &scripts);
                    }

                    Some(Runner::Makefile)
                }
                "package.json" => {
                    if !script_map.has_js_runner() {
                        Some(Runner::Npm)
                    } else {
                        None
                    }
                }
                _ => None,
            };

            let runner = unwrap_or_continue!(runner);
            if let Language::Javascript = Language::from(&runner) {
                let path = current_level.join("package.json");
                if let Ok(scripts) = parse_package_json_scripts(&path) {
                    script_map.add_scripts(&runner, &scripts);
                    script_map.add_runner(&Runner::Npm);
                }
            }

            script_map.add_runner(&runner);
        }
    }

    Ok(script_map)
}
