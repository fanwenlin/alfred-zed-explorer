use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,
}

pub fn is_project(path: &Path) -> bool {
    if !path.is_dir() {
        return false;
    }

    // Skip common non-project directories
    let name = path.file_name().unwrap_or_default();
    let name = name.to_string_lossy();

    if name.starts_with('.')
        || name == "node_modules"
        || name == "target"
        || name == "dist"
        || name == "build"
        || name == "__pycache__"
        || name == "vendor"
    {
        return false;
    }

    // Check for project indicators
    if path.join(".git").is_dir() {
        return true;
    }

    let indicators = [
        "package.json",     // Node.js
        "Cargo.toml",       // Rust
        "pyproject.toml",   // Python
        "requirements.txt", // Python
        "go.mod",           // Go
        "composer.json",    // PHP
        "Gemfile",          // Ruby
        "Makefile",         // C/C++
        "CMakeLists.txt",   // C/C++
        "*.xcodeproj",      // Xcode
        "*.xcworkspace",    // Xcode
        "*.sln",            // Visual Studio
        "pom.xml",          // Java Maven
        "build.gradle",     // Java Gradle
    ];

    for indicator in &indicators {
        if indicator.contains('*') {
            // Wildcard match (for .xcodeproj, etc.)
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    if let Some(name) = entry.file_name().to_str() {
                        if wildcard_match(name, indicator) {
                            return true;
                        }
                    }
                }
            }
        } else if path.join(indicator).is_file() {
            return true;
        }
    }

    false
}

fn wildcard_match(name: &str, pattern: &str) -> bool {
    let pattern = pattern.replace('*', "");
    name.ends_with(&pattern)
}

pub fn detect_projects(dirs: &[PathBuf], max_depth: usize) -> Result<Vec<Project>> {
    let mut projects = Vec::new();
    let mut seen_paths = std::collections::HashSet::new();

    for base_dir in dirs {
        if !base_dir.exists() {
            continue;
        }

        // Search for projects
        for entry in WalkDir::new(base_dir)
            .max_depth(max_depth)
            .into_iter()
            .filter_entry(|e| {
                let name = e.file_name().to_string_lossy();
                !name.starts_with('.')
                    && name != "node_modules"
                    && name != "target"
                    && name != "__pycache__"
            })
            .flatten()
        {
            let path = entry.path();

            if is_project(path) {
                let canonical_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
                let path_str = canonical_path.to_string_lossy().to_string();

                if !seen_paths.contains(&path_str) {
                    seen_paths.insert(path_str);

                    let name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("Unknown")
                        .to_string();

                    projects.push(Project {
                        name,
                        path: canonical_path,
                    });
                }
            }
        }
    }

    // Sort projects by name
    projects.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    Ok(projects)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_is_project_git() {
        let temp_dir = TempDir::new().unwrap();
        let git_dir = temp_dir.path().join(".git");
        fs::create_dir(git_dir).unwrap();

        assert!(is_project(temp_dir.path()));
    }

    #[test]
    fn test_is_project_package_json() {
        let temp_dir = TempDir::new().unwrap();
        let package_json = temp_dir.path().join("package.json");
        fs::write(package_json, "{}").unwrap();

        assert!(is_project(temp_dir.path()));
    }

    #[test]
    fn test_is_not_project() {
        let temp_dir = TempDir::new().unwrap();
        assert!(!is_project(temp_dir.path()));
    }

    #[test]
    fn test_skip_hidden_dirs() {
        let temp_dir = TempDir::new().unwrap();
        let hidden_dir = temp_dir.path().join(".hidden");
        fs::create_dir(&hidden_dir).unwrap();

        let package_json = hidden_dir.join("package.json");
        fs::write(package_json, "{}").unwrap();

        assert!(!is_project(&hidden_dir));
    }
}
