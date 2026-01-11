use anyhow::Result;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::env;
use zed_workspace_explorer::{
    detect_projects, get_project_directories, get_recent_projects, AlfredIcon, AlfredItem,
    AlfredOutput,
};

fn main() -> Result<()> {
    let query = env::args().nth(1).unwrap_or_default();
    let matcher = SkimMatcherV2::default();

    let mut output = AlfredOutput::new();

    // Try to get projects from Zed database
    match get_recent_projects() {
        Ok(recent_projects) => {
            // Filter by query and convert to our format
            let filtered: Vec<_> = recent_projects
                .into_iter()
                .filter(|project| {
                    if query.is_empty() {
                        true
                    } else {
                        let name = project
                            .path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("");
                        let score = matcher.fuzzy_match(name, &query).or_else(|| {
                            matcher.fuzzy_match(&project.path.to_string_lossy(), &query)
                        });
                        score.is_some()
                    }
                })
                .collect();

            // If we have recent projects, show them
            if !filtered.is_empty() {
                for project in filtered.iter().take(50) {
                    // Limit to 50 items
                    let icon = zed_workspace_explorer::get_icon_for_project(&project.path);
                    let name = project
                        .path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("Unknown");
                    let path = project.path.to_string_lossy();

                    let timestamp_text = if let Some(ts) = project.timestamp {
                        let date = chrono::DateTime::from_timestamp(ts, 0);
                        if let Some(d) = date {
                            format!(" • {}", d.format("%Y-%m-%d %H:%M"))
                        } else {
                            String::new()
                        }
                    } else {
                        String::new()
                    };

                    output.add_item(AlfredItem {
                        uid: path.to_string(),
                        item_type: "file".to_string(),
                        title: format!("{} {}{}", icon, name, timestamp_text),
                        subtitle: path.to_string(),
                        arg: path.to_string(),
                        autocomplete: name.to_string(),
                        icon: AlfredIcon {
                            icon_type: "fileicon".to_string(),
                            path: path.to_string(),
                        },
                    });
                }
            }
        }
        Err(e) => {
            eprintln!("Warning: Could not get recent projects from Zed DB: {}", e);
        }
    }

    // If we still don't have items or if we're searching, also search custom directories
    if output.items.is_empty() || !query.is_empty() {
        let dirs = get_project_directories();
        let projects = detect_projects(&dirs, 2)?; // Only search 2 levels deep for recent projects

        let filtered: Vec<_> = if query.is_empty() {
            projects.into_iter().collect()
        } else {
            projects
                .into_iter()
                .filter(|project| {
                    let score = matcher
                        .fuzzy_match(&project.name, &query)
                        .or_else(|| matcher.fuzzy_match(&project.path.to_string_lossy(), &query));
                    score.is_some()
                })
                .collect()
        };

        if !filtered.is_empty() {
            // Add a separator if we have both recent and custom directory projects
            if !output.items.is_empty() {
                output.add_item(AlfredItem {
                    uid: "separator".to_string(),
                    item_type: "default".to_string(),
                    title: "—— Custom Directories ——".to_string(),
                    subtitle: String::new(),
                    arg: String::new(),
                    autocomplete: String::new(),
                    icon: AlfredIcon {
                        icon_type: "default".to_string(),
                        path: String::new(),
                    },
                });
            }

            for project in filtered.into_iter().take(30) {
                // Limit to prevent too many items
                let icon = zed_workspace_explorer::get_icon_for_project(&project.path);
                let path_str = project.path.to_string_lossy();

                output.add_item(AlfredItem {
                    uid: path_str.to_string(),
                    item_type: "file".to_string(),
                    title: format!("{} {}", icon, project.name),
                    subtitle: path_str.to_string(),
                    arg: path_str.to_string(),
                    autocomplete: project.name,
                    icon: AlfredIcon {
                        icon_type: "fileicon".to_string(),
                        path: path_str.to_string(),
                    },
                });
            }
        }
    }

    if output.items.is_empty() {
        output.add_no_results(
            "No recent projects found",
            "Start working on projects or open folders in Zed",
        );
    }

    output.print()?;
    Ok(())
}
