pub struct Shell {
    pub current_dir: String,
    hide_cmd: bool,
}

impl Shell {
    pub fn new(current_dir: &str) -> Self {
        Self {
            current_dir: current_dir.to_string(),
            hide_cmd: false,
        }
    }
    pub fn run_status(&self, cmd: &str) {
        let prefix = if self.current_dir == "scripts" {
            "cd .. && "
        } else {
            ""
        };

        if !self.hide_cmd {
            println!();
            println!("Running: {}{}", prefix, cmd);
        }
        let status = std::process::Command::new("bash")
            .arg("-c")
            .arg(format!("set -e\n{prefix}{cmd}"))
            .status()
            .unwrap();

        if !status.success() {
            std::process::exit(1);
        }
    }
    pub fn run_output(&self, cmd: &str) -> String {
        let prefix = if self.current_dir == "scripts" {
            "cd .. && "
        } else {
            ""
        };

        if !self.hide_cmd {
            println!();
            println!("Running: {}{}", prefix, cmd);
        }

        let output = std::process::Command::new("bash")
            .arg("-c")
            .arg(format!("set -e\n{prefix}{cmd}"))
            .output()
            .unwrap();

        if !output.status.success() {
            let stderr = String::from_utf8(output.stderr).unwrap();
            println!("stderr: {}", stderr);
            std::process::exit(1);
        }

        String::from_utf8(output.stdout).unwrap()
    }
}

pub fn doc(_current_dir: &Shell) {
    // This is failing currently
    // shell.run_status("cargo doc --release", current_dir);
}
