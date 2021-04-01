use std::{
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    let original_path = std::env::var("PATH").unwrap_or_default();
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let virtual_env = manifest_dir.join("env");
    let path = format!("{}:{}", virtual_env.display(), original_path);

    let state = State {
        manifest_dir,
        original_path,
        virtual_env,
        path,
    };

    let venv = state.virtual_env.as_os_str();

    state.python_cmd("venv", &[venv]);
    state.python_cmd("pip", &["install", "maturin"]);
    state.run(Command::new("maturin").arg("develop"));

    let tests = state
        .manifest_dir
        .join("tests")
        .join("integration_tests.py");
    state.python_cmd("unittest", &["--verbose".as_ref(), tests.as_os_str()]);
}

#[derive(Debug)]
struct State {
    manifest_dir: &'static Path,
    original_path: String,
    virtual_env: PathBuf,
    path: String,
}

impl State {
    fn python_cmd<S>(&self, module: &str, args: &[S])
    where
        S: AsRef<OsStr>,
    {
        let python = if self.virtual_env.exists() {
            self.virtual_env
                .join("bin")
                .join("python3")
                .into_os_string()
        } else {
            OsString::from("python3")
        };

        let mut cmd = Command::new(python);
        cmd.arg("-m").arg(module).args(args);

        self.run(&mut cmd);
    }

    fn run(&self, cmd: &mut Command) {
        cmd.env("VIRTUAL_ENV", &self.virtual_env)
            .env("PATH", &self.path)
            .current_dir(&self.manifest_dir);

        eprintln!("Executing {:?}", cmd);

        let status = cmd.status().expect("Unable to invoke the command");

        assert!(
            status.success(),
            "The command failed with exit code {}",
            status.code().unwrap_or(1)
        );
    }
}
