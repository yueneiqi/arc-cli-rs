use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "arc-cli-rs")]
#[command(about = "A CLI for the Arc Browser (Rust implementation)", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show Arc version
    ArcVersion,

    /// List spaces
    #[command(alias = "ls")]
    ListSpaces,

    /// Select a space
    #[command(alias = "s")]
    SelectSpace {
        /// Space ID to select
        space_id: i32,
    },

    /// Select a space by name
    #[command(alias = "sn")]
    SelectSpaceName {
        /// Space name to select
        space_name: String,
    },

    /// List tabs
    #[command(alias = "lt")]
    ListTabs,

    /// Open URL in a new tab
    #[command(alias = "nt")]
    NewTab {
        /// URL to open
        url: String,
    },

    /// Select tab
    #[command(alias = "st")]
    SelectTab {
        /// Window ID
        window_id: i32,
        /// Tab ID
        tab_id: i32,
    },

    /// Select tab by name
    #[command(alias = "stn")]
    SelectTabName {
        /// Tab name to select
        tab_name: String,

        /// Case-sensitive matching
        #[arg(short = 'c', long)]
        case_sensitive: bool,

        /// Exact match only (no partial matching)
        #[arg(short = 'e', long)]
        exact: bool,
    },

    /// Reload tab
    #[command(alias = "rt")]
    ReloadTab {
        /// Window ID
        window_id: i32,
        /// Tab ID
        tab_id: i32,
    },

    /// Close tab
    #[command(alias = "ct")]
    CloseTab {
        /// Window ID
        window_id: i32,
        /// Tab ID
        tab_id: i32,
    },

    /// Open URL in a new Little Arc window
    #[command(alias = "nla")]
    NewLittleArc {
        /// URL to open
        url: String,
    },
}
