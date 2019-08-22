use std::process::{Command, Stdio};

fn main() {
    // Install cargo web if it is not installed.
    if Command::new("cargo")
        .arg("web")
        .output()
        .expect("Could not run cargo")
        .status
        .code()
        .expect("Error running cargo web.")
        == 101
    {
        println!("Install cargo-web");
        println!("-----------------");
        let output = Command::new("cargo")
            .arg("install")
            .arg("--color")
            .arg("always")
            .arg("cargo-web")
            .stderr(Stdio::inherit())
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .output()
            .expect("Could not install cargo-web");

        let output = String::from_utf8_lossy(&output.stdout).into_owned();

        println!("{}", output);
    }

   let output = Command::new("ls")
        .stderr(Stdio::inherit())
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .output()
        .expect("Could not build with cargo-web.");

         let output = String::from_utf8_lossy(&output.stdout).into_owned();

        println!("{}", output);


    Command::new("cargo")
        .arg("web")
        .arg("build")
        .arg("--example")
        .arg("widgets")
        .stderr(Stdio::inherit())
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .output()
        .expect("Could not build with cargo-web.");
}
