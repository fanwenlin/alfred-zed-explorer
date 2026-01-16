use anyhow::Result;
use rusqlite::Connection;
use std::path::{Path, PathBuf};

const RECENT_PROJECTS_KEY: &str = "recent_projects";
const KV_TABLE_NAME: &str = "kv_store"; // Zed uses 'kv_store', not 'kv'

#[derive(Debug, Clone)]
pub struct ZedRecentProject {
    pub path: PathBuf,
    pub timestamp: Option<i64>,
    pub remote_info: Option<RemoteInfo>,
}

#[derive(Debug, Clone)]
pub struct RemoteInfo {
    pub connection_id: i64,
    pub kind: String,
    pub host: Option<String>,
}

pub fn get_zed_config_dir() -> Result<PathBuf> {
    if let Some(home) = dirs::home_dir() {
        let macos_path = home.join("Library/Application Support/Zed");
        if macos_path.exists() {
            return Ok(macos_path);
        }

        let linux_path = home.join(".local/share/zed");
        if linux_path.exists() {
            return Ok(linux_path);
        }

        let flatpak_path = home.join(".var/app/dev.zed.Zed/data/zed");
        if flatpak_path.exists() {
            return Ok(flatpak_path);
        }
    }

    Err(anyhow::anyhow!("Could not find Zed config directory"))
}

pub fn discover_db_paths(config_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut db_paths = Vec::new();
    let db_parent = config_dir.join("db");

    if !db_parent.exists() {
        return Ok(db_paths);
    }

    // Read db directory
    for entry in std::fs::read_dir(&db_parent)?.flatten() {
        let path = entry.path();

        if path.is_dir() {
            let dir_name = entry.file_name();
            let dir_name_str = dir_name.to_string_lossy();

            // Match pattern: digit(s) followed by "-" and then (preview|global|stable)
            if let Ok(metadata) = std::fs::metadata(&path) {
                if metadata.is_dir() && is_valid_db_directory(&dir_name_str) {
                    // Check for both 'db' and 'db.sqlite' file names
                    let possible_files = ["db", "db.sqlite"];
                    for filename in &possible_files {
                        let db_file_path = path.join(filename);
                        if db_file_path.exists() && db_file_path.is_file() {
                            db_paths.push(db_file_path);
                            break; // Use first found file
                        }
                    }
                }
            }
        }
    }

    Ok(db_paths)
}

fn is_valid_db_directory(dir_name: &str) -> bool {
    // Check if directory name matches pattern: <digits>-<suffix>
    // where suffix is one of: preview, global, stable
    if let Some(dash_pos) = dir_name.find('-') {
        let prefix = &dir_name[..dash_pos];
        let suffix = &dir_name[dash_pos + 1..];

        // Check if prefix is all digits
        if prefix.chars().all(|c| c.is_ascii_digit()) {
            // Check if suffix is valid
            matches!(suffix, "preview" | "global" | "stable")
        } else {
            false
        }
    } else {
        false
    }
}

pub fn get_recent_projects() -> Result<Vec<ZedRecentProject>> {
    let config_dir = get_zed_config_dir()?;
    let db_paths = discover_db_paths(&config_dir)?;

    let mut all_projects = Vec::new();
    let mut seen_paths = std::collections::HashSet::new();

    for db_path in db_paths {
        match get_recent_projects_from_db(&db_path) {
            Ok(mut projects) => {
                for project in projects.drain(..) {
                    let path_str = project.path.to_string_lossy().to_string();

                    // Skip duplicate paths
                    if seen_paths.contains(&path_str) {
                        continue;
                    }

                    // For local projects, check if path exists
                    // For remote projects, always include them (we can't check remote path existence)
                    if project.remote_info.is_some() || project.path.exists() {
                        seen_paths.insert(path_str);
                        all_projects.push(project);
                    }
                }
            }
            Err(e) => {
                eprintln!("Warning: Failed to read {:?}: {}", db_path, e);
            }
        }
    }

    // Sort by timestamp (newest first), then by path
    all_projects.sort_by(|a, b| {
        b.timestamp
            .cmp(&a.timestamp)
            .then_with(|| b.path.cmp(&a.path))
    });

    Ok(all_projects)
}

fn get_recent_projects_from_db(db_path: &Path) -> Result<Vec<ZedRecentProject>> {
    let conn = Connection::open(db_path)?;

    // Try workspaces table first (newer Zed versions)
    match get_recent_projects_from_workspaces(&conn) {
        Ok(projects) => {
            if !projects.is_empty() {
                return Ok(projects);
            }
            // Empty result from workspaces table, continue to fallback
        }
        Err(_e) => {
            // Workspaces table doesn't exist or query failed, silently try fallback
        }
    }

    // Fall back to kv_store method for backward compatibility
    get_recent_projects_from_kv_store(db_path)
}

fn get_recent_projects_from_workspaces(conn: &Connection) -> Result<Vec<ZedRecentProject>> {
    // First, fetch all remote connections to build a lookup map
    let mut remote_conn_map = std::collections::HashMap::new();
    let mut stmt = conn.prepare("SELECT id, kind, host FROM remote_connections")?;
    let remote_connections = stmt.query_map([], |row| {
        let id: i64 = row.get(0)?;
        let kind: String = row.get(1)?;
        let host: Option<String> = row.get(2)?;
        Ok((id, kind, host))
    })?;

    for conn_result in remote_connections {
        if let Ok((id, kind, host)) = conn_result {
            remote_conn_map.insert(id, RemoteInfo {
                connection_id: id,
                kind,
                host,
            });
        }
    }

    // Query the workspaces table
    let mut stmt = conn.prepare(
        "SELECT paths, timestamp, remote_connection_id FROM workspaces ORDER BY timestamp DESC"
    )?;

    let projects = stmt.query_map([], |row| {
        let paths_str: String = row.get(0)?;
        let timestamp_str: String = row.get(1)?;
        let remote_connection_id: Option<i64> = row.get(2)?;

        // Parse timestamp (format: "YYYY-MM-DD HH:MM:SS")
        let timestamp = chrono::NaiveDateTime::parse_from_str(&timestamp_str, "%Y-%m-%d %H:%M:%S")
            .ok()
            .map(|dt| dt.and_utc().timestamp());

        Ok((paths_str, timestamp, remote_connection_id))
    })?;

    let mut recent_projects = Vec::new();
    let mut seen_paths = std::collections::HashSet::new();

    for project_result in projects.flatten() {
        let (paths_str, timestamp, remote_connection_id) = project_result;

        // Get remote info if available
        let remote_info = remote_connection_id.and_then(|conn_id| remote_conn_map.get(&conn_id).cloned());

        // Split paths by | and handle each path
        for path in paths_str.split('|') {
            let path = path.trim();
            if !path.is_empty() {
                // Skip if we've already seen this path
                let path_key = if remote_info.is_some() {
                    format!("remote:{}: {}", remote_connection_id.unwrap_or(0), path)
                } else {
                    path.to_string()
                };

                if seen_paths.contains(&path_key) {
                    continue;
                }
                seen_paths.insert(path_key);

                recent_projects.push(ZedRecentProject {
                    path: PathBuf::from(path),
                    timestamp,
                    remote_info: remote_info.clone(),
                });
            }
        }
    }

    Ok(recent_projects)
}

fn get_recent_projects_from_kv_store(db_path: &Path) -> Result<Vec<ZedRecentProject>> {
    let conn = Connection::open(db_path)?;

    // Try both 'kv' and 'kv_store' table names for compatibility
    let table_names = ["kv", KV_TABLE_NAME];

    for table_name in &table_names {
        let query = format!("SELECT value FROM {} WHERE key = ?", table_name);
        let row_result = conn.query_row(&query, [RECENT_PROJECTS_KEY], |row| {
            let json_str: String = row.get(0)?;
            Ok(json_str)
        });

        if let Ok(json_str) = row_result {
            return parse_recent_projects_json(&json_str);
        }
        // Otherwise, continue to try next table
    }

    // If we get here, neither table worked
    Err(anyhow::anyhow!(
        "No recent projects found in database {:?} - Zed may not have tracked any projects yet",
        db_path
    ))
}

fn parse_recent_projects_json(json_str: &str) -> Result<Vec<ZedRecentProject>> {
    // Zed stores recent_projects as a JSON array of objects
    // Each object has: {"path": "...", "timestamp": 1234567890}
    let parsed: serde_json::Value = serde_json::from_str(json_str)?;

    let mut projects = Vec::new();

    if let Some(arr) = parsed.as_array() {
        for item in arr {
            if let Some(obj) = item.as_object() {
                if let Some(path_str) = obj.get("path").and_then(|v| v.as_str()) {
                    let path = PathBuf::from(path_str);

                    let timestamp = obj.get("timestamp").and_then(|v| v.as_i64());

                    // KV store doesn't have remote info, so set it to None
                    projects.push(ZedRecentProject { path, timestamp, remote_info: None });
                }
            }
        }
    }

    Ok(projects)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_db_directory() {
        assert!(is_valid_db_directory("0-preview"));
        assert!(is_valid_db_directory("123-global"));
        assert!(is_valid_db_directory("5-stable"));
        assert!(!is_valid_db_directory("preview"));
        assert!(!is_valid_db_directory("0-unknown"));
        assert!(!is_valid_db_directory("abc-preview"));
    }
}
