use std::{
    env,
    process::{Command, ExitStatus},
};

fn main() {
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

    if !output.status.success() || core_dumped(output.status) {
        let out = String::from_utf8_lossy(&output.stdout);
        let err = String::from_utf8_lossy(&output.stderr);
        panic!("error while compiling wasm:\nout:{out}\nerr:{err}\n");
    }
}

fn core_dumped(status: ExitStatus) -> bool {
    let dumped;

    #[cfg(target_family = "unix")]
    {
        use std::os::unix::process::ExitStatusExt;
        dumped = status.core_dumped();
    }

    #[cfg(not(target_family = "unix"))]
    {
        dumped = false;
    }

    dumped
}
