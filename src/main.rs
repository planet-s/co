use std::process::{Command, Stdio};

fn main() {
    if let Ok(output) = Command::new("cargo").arg("web").output() {
        if !output.status.success() {
            Command::new("cargo")
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
    } else {
        Command::new("cargo")
            .arg("install")
            .arg("cargo-web")
            .output()
            .expect("Could not install cargo-web");
    }
}
