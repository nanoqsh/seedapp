fn main() {
    use std::{env, process::Command};

    println!("cargo:rerun-if-changed=web/");

    let debug = env::var("DEBUG")
        .map(|var| var == "true")
        .unwrap_or_default();

    let output = Command::new("wasm-pack")
        .args([
            "build",
            "--target",
            "web",
            "--out-dir",
            "../static/pkg/",
            "--no-typescript",
        ])
        .args(debug.then_some("--dev"))
        .arg("web/")
        .output()
        .expect("build wasm");

    if !output.status.success() {
        let out = String::from_utf8_lossy(&output.stdout);
        let err = String::from_utf8_lossy(&output.stderr);
        panic!("error while compiling wasm:\nout:{out}\nerr:{err}\n");
    }
}
