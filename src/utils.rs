use std::process::{Command, Stdio};
use std::path::PathBuf;
use std::env;
use std::fs;

use std::fs::File;
use std::error::Error;

/// Select input between `txt_file`, `input_arg` and `default`, and converts it to string
pub fn select_input(input_arg: Option<&str>, default: &str) -> String {
    let hex_string = match input_arg {
        Some(arg) => {
            if arg.ends_with(".txt") {
                // Treat as filename
                fs::read_to_string(arg)
                    .unwrap_or_else(|_| "".to_string())
                    .trim()
                    .to_string()
            } else {
                // Treat as literal string
                arg.trim().to_string()
            }
        }
        None => "".to_string(),
    };

    if hex_string.is_empty() {
        default.to_string()
    } else {
        hex_string
    }
}


pub fn read_file(file_path: &str) -> Result<File, Box<dyn Error>> {
    let file = File::open(file_path)?; // ✅ auto-converts, no map_err needed
    Ok(file)
}

/// Run a python script from rust file
pub fn run_python(script_path: &str) {
    // Determine the Python executable based on OS
    let python_exe = if cfg!(target_os = "windows") {
        "python"
    } else {
        "python3"
    };

    // Get current executable directory
    let exe_path: PathBuf = env::current_exe().expect("Failed to get current exe path");
    let script_dir = exe_path.parent().expect("Failed to get parent dir");

    println!("=== Running Python script {} ===", script_path);

    let output = Command::new(python_exe)
        .arg(script_path)
        .current_dir(script_dir) // ensures relative paths work
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn python process")
        .wait_with_output()
        .expect("Failed to wait on python process");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    if !output.stderr.is_empty() {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}
