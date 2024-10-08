use crate::parsers::{parse_makefile_targets, parse_package_json_scripts};
use crate::runners::{Language, Runner};
use std::collections::HashMap;
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
    runners: [bool; 9],
    scripts: [Vec<String>; 9],
    map: HashMap<String,Runner>
}

impl ScriptMap {
    fn has_runner(&self, runner: Runner) -> bool {
        let i: usize = runner.into();
        self.runners[i]
    }

    fn has_js_runner(&self) -> bool {
        self.runners[0..=4].iter().any(|&v| v)
    }

    fn add_runner(&mut self,runner: &Runner) {
        let i: usize = runner.clone().into();
        
        self.runners[i] = true;
        for command in runner.default_commands() {
            let command = command.to_string();
            
            // Prevent defaults from overriding higher priority scripts
            if !self.map.contains_key(&command) {
                self.map.insert(command.to_string(), runner.clone());
            }
        }
    }

    fn add_scripts<V, S>(&mut self, runner: &Runner, scripts: V)
    where
        V: AsRef<[S]>,
        S: AsRef<str>,
    {
        let i: usize = runner.clone().into();

        self.runners[i] = true;
        for script in scripts.as_ref() {
            let script = script.as_ref();

            self.scripts[i].push(script.to_string());
            self.map.insert(script.to_string(), runner.clone());
        }
    }

    pub fn display(&self) -> String {
        let mut buffer = String::new();
        let (width, _) = term_size::dimensions().unwrap_or((999, 0));

        for (i, val) in self.runners.iter().enumerate() {
            if !val {
                continue;
            }

            let runner = Runner::from_usize(i).unwrap();

            buffer += (&runner).into();
            buffer += ":";

            let scripts = &self.scripts[i];
            let mut len = 0;

            if !scripts.is_empty() {
                buffer += "\n    ";
                len += 4;
            }

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

            if !runner.default_commands().is_empty() {
                buffer += "\n    ";
                buffer += (&runner).into();
                buffer += "'s default commands, view with (";
                buffer += (&runner).into();
                buffer += " --help)";
            }

            buffer += "\n"
        }

        buffer
    }

    pub fn no_runners(&self) -> bool {
        self.runners.iter().all(|f| !f)
    }

    pub fn find_runner(&self, script: &str) -> Option<&Runner> {
        let command = Runner::unalias_command(script);
        self.map.get(command)
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
                if script_map.has_js_runner() {
                    continue;
                }

                let path = current_level.join("package.json");
                if let Ok(scripts) = parse_package_json_scripts(&path) {
                    script_map.add_scripts(&runner, &scripts);
                }
            }

            script_map.add_runner(&runner);
        }
    }

    Ok(script_map)
}
