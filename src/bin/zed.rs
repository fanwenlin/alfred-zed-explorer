use anyhow::Result;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::collections::HashSet;
use std::env;
use zed_workspace_explorer::{
    detect_projects, get_project_directories, get_recent_projects, AlfredIcon, AlfredItem,
    AlfredOutput,
};

fn main() -> Result<()> {
    let query = env::args().nth(1).unwrap_or_default();
    let matcher = SkimMatcherV2::default();

    let mut output = AlfredOutput::new();
    let mut recent_paths_set = HashSet::new();

    // Step 1: Get recent projects from Zed DB
    let mut has_recent = false;
    match get_recent_projects() {
        Ok(recent_projects) => {
            // Filter recent projects by query
            let filtered_recent: Vec<_> = if query.is_empty() {
                recent_projects
            } else {
                recent_projects
                    .into_iter()
                    .filter(|project| {
                        let name = project
                            .path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("");
                        let score = matcher.fuzzy_match(name, &query).or_else(|| {
                            matcher.fuzzy_match(&project.path.to_string_lossy(), &query)
                        });
                        score.is_some()
                    })
                    .collect()
            };

            // Add filtered recent projects (up to 50)
            if !filtered_recent.is_empty() {
                has_recent = true;
                for project in filtered_recent.iter().take(50) {
                    // Track all recent paths for deduplication
                    let path_str = project.path.to_string_lossy().to_string();
                    recent_paths_set.insert(path_str.clone());

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

    // Step 2: Scan custom directories for additional projects
    let dirs = get_project_directories();
    let all_dir_projects = detect_projects(&dirs, 2)?; // 2 levels deep like zrecent

    // Filter out projects that already appear in recent list
    let filtered_dir_projects: Vec<_> = all_dir_projects
        .into_iter()
        .filter(|project| {
            let path_str = project.path.to_string_lossy().to_string();
            !recent_paths_set.contains(&path_str)
        })
        .collect();

    // Step 3: Apply query filter to directory projects
    let query_filtered_dir: Vec<_> = if query.is_empty() {
        filtered_dir_projects
    } else {
        filtered_dir_projects
            .into_iter()
            .filter(|project| {
                let score = matcher
                    .fuzzy_match(&project.name, &query)
                    .or_else(|| matcher.fuzzy_match(&project.path.to_string_lossy(), &query));
                score.is_some()
            })
            .collect()
    };

    // Step 4: Add separator if we have both recent and directory results
    if has_recent && !query_filtered_dir.is_empty() {
        output.add_item(AlfredItem {
            uid: "separator-dir".to_string(),
            item_type: "default".to_string(),
            title: "—— Directory Projects ——".to_string(),
            subtitle: String::new(),
            arg: String::new(),
            autocomplete: String::new(),
            icon: AlfredIcon {
                icon_type: "default".to_string(),
                path: String::new(),
            },
        });
    }

    // Step 5: Add directory projects (up to 30)
    if !query_filtered_dir.is_empty() {
        for project in query_filtered_dir.iter().take(30) {
            let icon = zed_workspace_explorer::get_icon_for_project(&project.path);
            let path_str = project.path.to_string_lossy();

            output.add_item(AlfredItem {
                uid: path_str.to_string(),
                item_type: "file".to_string(),
                title: format!("{} {}", icon, project.name),
                subtitle: path_str.to_string(),
                arg: path_str.to_string(),
                autocomplete: project.name.clone(),
                icon: AlfredIcon {
                    icon_type: "fileicon".to_string(),
                    path: path_str.to_string(),
                },
            });
        }
    }

    // Step 6: Handle empty results
    if output.items.is_empty() {
        if query.is_empty() {
            output.add_no_results(
                "No projects found",
                "Open projects in Zed or add directories to PROJECT_DIRS",
            );
        } else {
            output.add_no_results(
                "No projects match your search",
                "Try a different search term",
            );
        }
    }

    output.print()?;
    Ok(())
}
