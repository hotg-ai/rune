use std::{
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    let runner = Runner::from_env();
    let venv = runner.virtual_env.as_os_str();

    runner.python("venv", &[venv]);
    runner.python("pip", &["install", "--upgrade", "pip"]);
    runner.python(
        "pip",
        &["install", "maturin", "--disable-pip-version-check"],
    );
    runner.maturin(&["develop"]);

    let tests = runner
        .manifest_dir
        .join("tests")
        .join("integration_tests.py");
    runner.python("unittest", &["--verbose".as_ref(), tests.as_os_str()]);
}

#[derive(Debug)]
struct Runner {
    manifest_dir: &'static Path,
    virtual_env: PathBuf,
    path: String,
}

impl Runner {
    fn from_env() -> Self {
        let original_path = std::env::var("PATH").unwrap_or_default();
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let virtual_env = manifest_dir.join("env");
        let path = format!("{}:{}", virtual_env.display(), original_path);

        Runner {
            manifest_dir,
            virtual_env,
            path,
        }
    }

    fn python<S>(&self, module: &str, args: &[S])
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

    fn maturin<S>(&self, args: &[S])
    where
        S: AsRef<OsStr>,
    {
        let maturin = self.virtual_env.join("bin").join("maturin");
        let mut cmd = Command::new(maturin);
        cmd.args(args)
            .env("CARGO_TARGET_DIR", self.manifest_dir.join("target"));

        self.run(&mut cmd);
    }

    fn run(&self, cmd: &mut Command) {
        cmd.env_clear()
            .env("VIRTUAL_ENV", &self.virtual_env)
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
