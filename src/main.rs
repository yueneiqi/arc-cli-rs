mod applescript;
mod arc;
mod cli;
mod types;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};
use utils::sanitize_url;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // If no command is provided, show help
    let Some(command) = cli.command else {
        Cli::parse_from(&["arc-cli-rs", "--help"]);
        return;
    };

    let result = match command {
        Commands::ArcVersion => {
            match arc::get_version().await {
                Ok(version) => {
                    println!("Arc Browser version: {}", version);
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
        Commands::ListSpaces => {
            match arc::get_spaces().await {
                Ok(spaces) => {
                    println!("Arc spaces:\n");
                    for space in spaces {
                        println!("{}: {}", space.id, space.title);
                    }
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
        Commands::SelectSpace { space_id } => {
            arc::select_space(space_id).await
        }
        Commands::SelectSpaceName { space_name } => {
            match arc::get_spaces().await {
                Ok(spaces) => {
                    let space = spaces
                        .iter()
                        .find(|s| s.title.to_lowercase() == space_name.to_lowercase());

                    match space {
                        Some(s) => arc::select_space(s.id).await,
                        None => {
                            eprintln!("Space with name \"{}\" not found", space_name);
                            Err(format!("Space \"{}\" not found", space_name))
                        }
                    }
                }
                Err(e) => Err(e),
            }
        }
        Commands::ListTabs => {
            match arc::get_tabs().await {
                Ok(tabs) => {
                    println!("Arc tabs:\n");
                    for tab in tabs {
                        println!(
                            "{}, {}, {}, {}, {:?}",
                            tab.window_id, tab.tab_id, tab.title, tab.url, tab.location
                        );
                    }
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
        Commands::NewTab { url } => {
            let sanitized_url = sanitize_url(&url);
            arc::new_tab(&sanitized_url).await
        }
        Commands::SelectTab { window_id, tab_id } => {
            arc::select_tab(window_id, tab_id).await
        }
        Commands::SelectTabName { tab_name, case_sensitive, exact } => {
            match arc::get_tabs().await {
                Ok(tabs) => {
                    let tab = tabs.iter().find(|t| {
                        let title = &t.title;
                        let name = &tab_name;

                        match (case_sensitive, exact) {
                            (true, true) => title == name,
                            (true, false) => title.contains(name),
                            (false, true) => title.to_lowercase() == name.to_lowercase(),
                            (false, false) => title.to_lowercase().contains(&name.to_lowercase()),
                        }
                    });

                    match tab {
                        Some(t) => arc::select_tab(t.window_id, t.tab_id).await,
                        None => {
                            eprintln!("Tab with name \"{}\" not found", tab_name);
                            Err(format!("Tab \"{}\" not found", tab_name))
                        }
                    }
                }
                Err(e) => Err(e),
            }
        }
        Commands::ReloadTab { window_id, tab_id } => {
            arc::reload_tab(window_id, tab_id).await
        }
        Commands::CloseTab { window_id, tab_id } => {
            arc::close_tab(window_id, tab_id).await
        }
        Commands::NewLittleArc { url } => {
            let sanitized_url = sanitize_url(&url);
            arc::new_little_arc(&sanitized_url).await
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
