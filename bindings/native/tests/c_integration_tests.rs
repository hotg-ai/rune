//! Use `cmake` to build and run our C test suite.

// use std::{path::Path, process::Command};

fn main() {
    // let project_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    // let test_dir = project_root.join("tests");
    // let build_dir = test_dir.join("build");

    // std::fs::create_dir_all(&build_dir)
    //     .expect("Unable to create the build directory");

    // let status = Command::new("cmake")
    //     .arg("..")
    //     .arg("-DCMAKE_MESSAGE_LOG_LEVEL=WARNING")
    //     .current_dir(&build_dir)
    //     .status()
    //     .expect("Unable to start cmake");
    // assert!(status.success(), "Cmake configure failed");

    // let status = Command::new("make")
    //     .current_dir(&build_dir)
    //     .status()
    //     .expect("Unable to start make");
    // assert!(status.success(), "build failed");

    // let status = Command::new("make")
    //     .arg("test")
    //     .current_dir(&build_dir)
    //     .status()
    //     .expect("Unable to start make");
    // assert!(status.success(), "test failed");
}
