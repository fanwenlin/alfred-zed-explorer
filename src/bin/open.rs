use anyhow::Result;
use std::env;
use std::process::Command;

fn main() -> Result<()> {
    let project_path = env::args().nth(1).unwrap_or_default();

    if project_path.is_empty() {
        eprintln!("Error: No project path provided");
        std::process::exit(1);
    }

    // Check if directory exists
    if !std::path::Path::new(&project_path).is_dir() {
        eprintln!("Error: Directory does not exist: {}", project_path);
        std::process::exit(1);
    }

    // Try to open with zed command
    let status = Command::new("zed")
        .arg(&project_path)
        .stderr(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .spawn();

    match status {
        Ok(mut child) => {
            // Detach the process and exit immediately
            // Alfred workflow scripts should not wait for the editor to close
            let _ = child.wait();
        }
        Err(e) => {
            // If zed command is not found, try alternative approaches
            eprintln!("Error: Failed to open with 'zed' command: {}", e);
            eprintln!();
            eprintln!("Please ensure:");
            eprintln!("1. Zed is installed from https://zed.dev/");
            eprintln!("2. The Zed CLI is installed:");
            eprintln!("   - Open Zed");
            eprintln!("   - Press Cmd+Shift+P");
            eprintln!("   - Run 'Install CLI'");
            eprintln!();
            eprintln!("Alternatively, you can manually open:");
            eprintln!("  {}", project_path);
            std::process::exit(1);
        }
    }

    Ok(())
}
