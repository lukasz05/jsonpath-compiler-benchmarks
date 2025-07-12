use std::env;
use eyre::{eyre, Context, Result};
use std::error::Error;
use std::path::Path;
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
    let jar_absolute_path = Path::new("./src/implementations/jsurferShim/lib/jsurferShim.jar").canonicalize()?;
    println!("cargo:rustc-env=LD_LIBRARY_PATH={java_home}/lib/server");
    println!(
        "cargo:rustc-env=RSONPATH_BENCH_JSURFER_SHIM_JAR_PATH={}",
        jar_absolute_path.display()
    );

    Ok(())
}

fn compile_queries() -> Result<()> {
    compile_cpp_file("src/cpp_helpers.cpp", false)?;
    compile_queries_from_file("src/ondemand_queries.txt", "simdjson-ondemand", false)?;
    compile_queries_from_file("src/ondemand_eager_filters_queries.txt", "simdjson-ondemand", true)?;
    compile_queries_from_file("src/dom_queries.txt", "simdjson-dom", false)?;
    Ok(())
}

fn compile_queries_from_file(queries_file_path: &str, target: &str, eager_filters: bool) -> Result<()> {
    let queries_file_stem = Path::new(&queries_file_path)
        .file_stem().unwrap().to_str().unwrap().to_string();
    let queries_code_file_path = Path::new("src").join(format!("{queries_file_stem}.cpp"))
        .to_str().unwrap().to_string();
    let bindings_file_path  = Path::new("src").join(format!("{queries_file_stem}_bindings.rs")
        .replace("-", "_")).to_str().unwrap().to_string();
    let mut jsonpath_compiler_command =
    Command::new("jsonpath-compiler");
    let mut jsonpath_compiler_command = jsonpath_compiler_command.arg("--target").arg(target)
        .arg("--output").arg(queries_code_file_path.clone())
        .arg("--rust-bindings").arg(bindings_file_path)
        .arg(queries_file_path);
    if eager_filters {
        jsonpath_compiler_command = jsonpath_compiler_command.arg("--eager-filter-evaluation");
    }
    let jsonpath_compiler_status = jsonpath_compiler_command.status()?;
    if !jsonpath_compiler_status.success() {
        return Err(eyre!("jsonpath-compiler execution failed with status code: {}", jsonpath_compiler_status));
    }
    compile_cpp_file(&queries_code_file_path, true)?;
    Ok(())
}

fn compile_cpp_file(file_path: &str, include_simdjson: bool) -> Result<()> {
    let file_stem = Path::new(&file_path).file_stem().unwrap().to_str().unwrap().to_string();
    let mut build = cc::Build::new();
    let mut build = build
        .file(file_path)
        .cpp(true)
        .std("c++20")
        .opt_level(3)
        .warnings(false);
    if include_simdjson {
        let simdjson_path = env::var("SIMDJSON_PATH")?;
        let simdjson_cpp_file = Path::new(&simdjson_path).join("simdjson.cpp")
            .to_str().unwrap().to_string();
        build = build.file(simdjson_cpp_file).include(simdjson_path);
    }
    build.compile(&file_stem);
    Ok(())
}
