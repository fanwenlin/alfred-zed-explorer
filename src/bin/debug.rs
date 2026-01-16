use anyhow::Result;
use zed_workspace_explorer::{get_recent_projects, zed_db};

fn main() -> Result<()> {
    println!("🔍 Searching for Zed recent projects in database...\n");

    // Debug: Show which paths we're checking
    println!("📂 Checking for Zed config directory...");
    match zed_db::get_zed_config_dir() {
        Ok(config_dir) => {
            println!("✅ Found Zed config directory:");
            println!("   {:?}\n", config_dir);

            println!("📂 Scanning for database files...");
            match zed_db::discover_db_paths(&config_dir) {
                Ok(db_paths) => {
                    if db_paths.is_empty() {
                        println!("⚠️  No database files found!");
                        println!();
                        println!("Checked path: {:?}/db/", config_dir);
                        println!();
                    } else {
                        println!("✅ Found {} database file(s):\n", db_paths.len());
                        for (i, path) in db_paths.iter().enumerate() {
                            println!("   {}. {:?}", i + 1, path);
                        }
                        println!();
                    }
                }
                Err(e) => {
                    println!("❌ Error scanning for DB paths: {}\n", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Could not find Zed config directory: {}\n", e);
        }
    }

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    match get_recent_projects() {
        Ok(projects) => {
            if projects.is_empty() {
                println!("⚠️  No recent projects found in Zed database");
                println!();
                println!("Possible reasons:");
                println!("  1. You haven't opened any projects in Zed yet");
                println!("  2. Zed database path is different than expected");
                println!("  3. Zed is not storing recent project data");
                println!();
                println!("Try opening a project in Zed, then run this command again.");
            } else {
                println!("✅ Found {} recent projects:\n", projects.len());

                for (i, project) in projects.iter().enumerate() {
                    let exists = if project.path.exists() {
                        "✓"
                    } else {
                        "✗ (missing)"
                    };

                    let name = project
                        .path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("Unknown");

                    let timestamp = if let Some(ts) = project.timestamp {
                        let date = chrono::DateTime::from_timestamp(ts, 0);
                        format!(
                            " [{}]",
                            date.map(|d| d.format("%Y-%m-%d %H:%M").to_string())
                                .unwrap_or_default()
                        )
                    } else {
                        String::new()
                    };

                    // Add remote indicator
                    let remote_indicator = if let Some(remote) = &project.remote_info {
                        let host = remote.host.as_deref().unwrap_or("remote");
                        format!("🌐 [{}] ", host)
                    } else {
                        String::new()
                    };

                    println!(
                        "{}. {} {}{}{}",
                        i + 1,
                        exists,
                        remote_indicator,
                        project.path.display(),
                        timestamp
                    );
                    println!("   └── {}{}\n", name, timestamp);
                }

                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
                println!("📊 Summary:");
                println!("   Total projects: {}", projects.len());
                println!(
                    "   Existing paths: {}",
                    projects.iter().filter(|p| p.path.exists()).count()
                );
                println!(
                    "   Missing paths:  {}",
                    projects.iter().filter(|p| !p.path.exists()).count()
                );
            }
        }
        Err(e) => {
            eprintln!("❌ Error reading Zed database: {}", e);
            eprintln!();
            eprintln!("Troubleshooting steps:");
            eprintln!("  1. Ensure Zed is installed");
            eprintln!("  2. Check Zed database exists:");
            eprintln!("     - macOS: ~/Library/Application\\ Support/Zed/db/");
            eprintln!("     - Linux: ~/.local/share/zed/db/");
            eprintln!("  3. Try opening a project in Zed first");
            eprintln!("  4. Check file permissions");
        }
    }

    Ok(())
}
