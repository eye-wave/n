use crate::runners::Runner;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct ScriptMap {
    runners: [bool; 9],
    scripts: [Vec<String>; 9],
    map: HashMap<String, Runner>,
}

impl ScriptMap {
    pub(super) fn has_runner(&self, runner: Runner) -> bool {
        let i: usize = runner.into();
        self.runners[i]
    }

    pub(super) fn has_js_runner(&self) -> bool {
        self.runners[0..=4].iter().any(|&v| v)
    }

    pub(super) fn add_runner(&mut self, runner: &Runner) {
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

    pub(super) fn add_scripts<V, S>(&mut self, runner: &Runner, scripts: V)
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

    pub fn display_all(&self) -> String {
        let mut commands = self
            .map
            .iter()
            .map(|(command, runner)| {
                let runner_name: &str = runner.into();
                format!("{runner_name}: {command}")
            })
            .collect::<Vec<_>>();

        commands.sort();

        println!("All avaiable commands are:");
        commands.join("\n")
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

            buffer += "\n    ";
            if runner.default_commands().is_empty() {
                buffer += (&runner).into();
                buffer += " doesn't have default commands."
            } else {
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
