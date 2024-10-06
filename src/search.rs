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

#[derive(Default, Debug)]
pub struct ScriptMap {
    buffer: [bool; 9],
    scripts: [Vec<String>; 9],
}

impl ScriptMap {
    fn has_runner(&self, runner: Runner) -> bool {
        let i: usize = runner.into();
        self.buffer[i]
    }

    fn has_js_runner(&self) -> bool {
        self.buffer[0..=4].iter().any(|&v| v)
    }

    fn add_scripts<V, S>(&mut self, runner: Runner, scripts: V)
    where
        V: AsRef<[S]>,
        S: AsRef<str>,
    {
        let i: usize = runner.into();

        self.buffer[i] = true;
        self.scripts[i] = scripts
            .as_ref()
            .iter()
            .map(|s| s.as_ref().to_string())
            .collect();
    }

    pub fn display(&self) -> String {
        let mut buffer = String::new();
        let (width, _) = term_size::dimensions().unwrap_or((999, 0));

        for (i, val) in self.buffer.iter().enumerate() {
            if !val {
                continue;
            }

            let runner = Runner::from_usize(i).unwrap();

            buffer += (&runner).into();
            buffer += ":\n    ";

            let scripts = &self.scripts[i];
            let mut len = 4;
            for (i, script) in scripts.iter().enumerate() {
                if (len + script.len() + 2) > width {
                    buffer += "\n    ";
                    len = 4;
                }

                buffer += script;
                if i < scripts.len() - 1 {
                    buffer += ", "
                }

                len += script.len() + 2;
            }

            buffer += &("\n".repeat((i < 8) as usize * 2))
        }

        buffer
    }

    pub fn no_runners(&self) -> bool {
        self.buffer.iter().all(|f| !f)
    }

    pub fn find_runner(&self, script: &str) -> Option<Runner> {
        let command = Runner::unalias_command(script);
        for (i, scripts) in self.scripts.iter().enumerate() {
            if scripts.contains(&command.to_string()) {
                return Runner::from_usize(i);
            }
        }

        None
    }
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
                "cargo.toml" | "cargo.lock" => {
                    script_map.add_scripts(Runner::Cargo, ["format", "lint", "test", "dev"]);
                    Some(Runner::Cargo)
                }
                "makefile" => {
                    if script_map.has_runner(Runner::Makefile) {
                        continue;
                    }

                    let path = current_level.join(filename);
                    if let Ok(scripts) = parse_makefile_targets(&path) {
                        script_map.add_scripts(Runner::Makefile, &scripts);
                    }

                    Some(Runner::Makefile)
                }
                "justfile" => Some(Runner::Justfile),
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
                if script_map.has_js_runner() {
                    continue;
                }

                let path = current_level.join("package.json");
                if let Ok(scripts) = parse_package_json_scripts(&path) {
                    script_map.add_scripts(runner, &scripts);
                }
            }
        }
    }

    Ok(script_map)
}
