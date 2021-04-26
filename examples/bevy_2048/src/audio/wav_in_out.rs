extern crate cpal;
extern crate hound;

mod record_wav;

pub fn in_and_out() {

    record_wav::record();

    fn cargo_manifest_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
    }

    fn microspeech_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("examples")
            .join("microspeech")
    }

    fn run_microspeech() {
        let microspeech_dir = microspeech_dir();
        let rune = microspeech_dir().join("microspeech.rune");
    
        let wav = cargo_manifest_dir()
            .join("recorded.wav");
    
        let mut cmd = Command::cargo_bin("rune").unwrap();
        cmd.arg("run")
            .arg(&rune)
            .arg(format!("--capability=sound:{}", wav.display()));
    }

    // Need to add something to read usefull line from json to find direction
    // Use that to control the game
    
}