use eyre::{eyre, Context, Result};
use std::error::Error;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    setup_jsurfer()?;
    compile_queries()?;

    Ok(())
}

fn setup_jsurfer() -> Result<()> {
    let gradlew_status = Command::new("./gradlew")
        .arg("shadowJar")
        .current_dir("./src/implementations/jsurferShim")
        .status()?;

    if !gradlew_status.success() {
        return Err(eyre!("gradlew execution failed with status code: {}", gradlew_status));
    }

    let java_home = std::env::var("JAVA_HOME").wrap_err("JAVA_HOME env variable not set")?;
    let jar_absolute_path =
        std::path::Path::new("./src/implementations/jsurferShim/lib/jsurferShim.jar").canonicalize()?;

    println!("cargo:rerun-if-changed=src/implementations/jsurferShim");
    println!("cargo:rustc-env=LD_LIBRARY_PATH={java_home}/lib/server");
    println!(
        "cargo:rustc-env=RSONPATH_BENCH_JSURFER_SHIM_JAR_PATH={}",
        jar_absolute_path.display()
    );

    Ok(())
}

fn compile_queries() -> Result<()> {
    cc::Build::new()
        .file("src/queries.cpp")
        .cpp(true)
        .std("c++20")
        .opt_level(3)
        .include("/opt/homebrew/Cellar/simdjson/3.10.1/include")
        .compile("queries");
    println!("cargo:rustc-link-search=/opt/homebrew/Cellar/simdjson/3.10.1/lib");
    println!("cargo:rustc-link-arg=-lsimdjson");
    Ok(())
}
