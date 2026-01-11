use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub mod project;
pub mod zed_db;

pub use project::{detect_projects, is_project, Project};
pub use zed_db::get_recent_projects;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlfredItem {
    pub uid: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub title: String,
    pub subtitle: String,
    pub arg: String,
    pub autocomplete: String,
    pub icon: AlfredIcon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlfredIcon {
    #[serde(rename = "type")]
    pub icon_type: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlfredOutput {
    pub items: Vec<AlfredItem>,
}

impl AlfredOutput {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: AlfredItem) {
        self.items.push(item);
    }

    pub fn print(&self) -> anyhow::Result<()> {
        println!("{}", serde_json::to_string(self)?);
        Ok(())
    }

    pub fn add_no_results(&mut self, title: &str, subtitle: &str) {
        self.items.push(AlfredItem {
            uid: "no-results".to_string(),
            item_type: "default".to_string(),
            title: title.to_string(),
            subtitle: subtitle.to_string(),
            arg: String::new(),
            autocomplete: String::new(),
            icon: AlfredIcon {
                icon_type: "default".to_string(),
                path: "/System/Library/CoreServices/CoreTypes.bundle/Contents/Resources/AlertNoteIcon.icns"
                    .to_string(),
            },
        });
    }
}

impl Default for AlfredOutput {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_icon_for_project(path: &Path) -> &'static str {
    if path.join(".git").is_dir() {
        if path.join("package.json").is_file() {
            "🟢" // Node.js
        } else if path.join("Cargo.toml").is_file() {
            "🟤" // Rust
        } else if path.join("pyproject.toml").is_file() || path.join("requirements.txt").is_file() {
            "🔵" // Python
        } else if path.join("go.mod").is_file() {
            "🟢" // Go
        } else if path.join("composer.json").is_file() {
            "🟣" // PHP
        } else if path.join("Gemfile").is_file() {
            "🔴" // Ruby
        } else {
            "🟠" // Generic Git
        }
    } else {
        "📁" // Generic folder
    }
}

pub fn get_project_directories() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    // Default directories
    let home = dirs::home_dir().unwrap_or_default();
    let default_paths = vec![
        "Projects",
        "Code",
        "Developer",
        "GitHub",
        "Development",
        "Sites",
        "workspace",
    ];

    for path in default_paths {
        dirs.push(home.join(path));
    }

    // Add custom directories from environment variable
    if let Ok(custom_dirs) = std::env::var("PROJECT_DIRS") {
        for dir in custom_dirs.split(',') {
            let dir = dir.trim();
            if !dir.is_empty() {
                dirs.push(PathBuf::from(dir));
            }
        }
    }

    dirs
}
