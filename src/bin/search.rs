use anyhow::Result;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::env;
use zed_workspace_explorer::{
    detect_projects, get_project_directories, AlfredIcon, AlfredItem, AlfredOutput,
};

fn main() -> Result<()> {
    let query = env::args().nth(1).unwrap_or_default();

    let dirs = get_project_directories();
    let projects = detect_projects(&dirs, 3)?;

    let mut output = AlfredOutput::new();
    let matcher = SkimMatcherV2::default();

    // Filter projects by query
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

    if filtered.is_empty() {
        output.add_no_results(
            "No projects found",
            "Try a different search term or add PROJECT_DIRS",
        );
    } else {
        for project in filtered {
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

    output.print()?;
    Ok(())
}
