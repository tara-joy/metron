use clap::{Parser, Subcommand, Args};

mod models;
mod storage;
mod managers;

use storage::Storage;
use managers::*;

#[derive(Parser)]
#[command(
    name = "metron",
    version = "1.0",
    author = "Tara Joy Hörtlehner",
    about = "Modular, cross-platform time-tracking app"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage categories (CRUD)
    #[command(subcommand)]
    Category(CategoryCommands),
    /// Manage tags (CRUD)
    #[command(subcommand)]
    Tag(TagCommands),
    /// Manage work sessions
    #[command(subcommand)]
    Session(SessionCommands),
    /// Show analytics
    Analysis(AnalysisArgs),
    /// Set total weekly quota
    SetQuota {
        /// Total weekly quota in hours
        hours: u32,
    },
}

#[derive(Subcommand)]
pub enum CategoryCommands {
    /// Create a new category
    Create {
        /// Name of the category
        name: String,
        /// Weekly quota in hours
        #[arg(short, long)]
        quota: u32,
    },
    /// List all categories
    List,
    /// Update an existing category's quota
    Update {
        name: String,
        #[arg(short, long)]
        quota: u32,
    },
    /// Delete a category
    Delete {
        name: String,
    },
}

#[derive(Subcommand)]
pub enum TagCommands {
    /// Create a new tag
    Create {
        name: String,
    },
    /// List all tags
    List,
    /// Delete a tag
    Delete {
        name: String,
    },
}

#[derive(Subcommand)]
pub enum SessionCommands {
    /// Start a new session
    Start {
        /// Title of the session
        title: String,
        /// Category name
        category: String,
        /// Tag names
        #[arg(short, long)]
        tags: Vec<String>,
        /// Duration in minutes (must be multiple of 15)
        #[arg(short, long)]
        duration: u32,
    },
    /// End a session early (will round down to nearest 15 min)
    End {
        /// Session ID
        id: String,
    },
    /// List all sessions
    List,
    /// Delete a session
    Delete {
        id: String,
    },
}

#[derive(Args)]
pub struct AnalysisArgs {
    /// Time period to analyze (day, week, month, year)
    #[arg(short, long, default_value = "week")]
    period: String,
    /// Filter by category
    #[arg(short, long)]
    category: Option<String>,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut storage = Storage::new("metron_data.json")?;

    match cli.command {
        Commands::Category(cmd) => {
            let mut manager = CategoryManager::new(&mut storage);
            match cmd {
                CategoryCommands::Create { name, quota } => {
                    manager.create_category(name, quota)?;
                }
                CategoryCommands::List => {
                    manager.list_categories()?;
                }
                CategoryCommands::Update { name, quota } => {
                    manager.update_category(name, quota)?;
                }
                CategoryCommands::Delete { name } => {
                    manager.delete_category(name)?;
                }
            }
        },
        Commands::Tag(cmd) => {
            let mut manager = TagManager::new(&mut storage);
            match cmd {
                TagCommands::Create { name } => {
                    manager.create_tag(name)?;
                }
                TagCommands::List => {
                    manager.list_tags()?;
                }
                TagCommands::Delete { name } => {
                    manager.delete_tag(name)?;
                }
            }
        },
        Commands::Session(cmd) => {
            let mut manager = SessionManager::new(&mut storage);
            match cmd {
                SessionCommands::Start { title, category, tags, duration } => {
                    manager.start_session(title, category, tags, duration)?;
                }
                SessionCommands::End { id } => {
                    manager.end_session(id)?;
                }
                SessionCommands::List => {
                    manager.list_sessions()?;
                }
                SessionCommands::Delete { id } => {
                    manager.delete_session(id)?;
                }
            }
        },
        Commands::Analysis(args) => {
            let manager = AnalysisManager::new(&storage);
            manager.generate_analysis(args.period, args.category)?;
        },
        Commands::SetQuota { hours } => {
            let data = storage.get_data_mut();
            data.total_weekly_quota = Some(hours);
            storage.save()?;
            println!("✓ Set total weekly quota to {}h", hours);
        }
    }

    Ok(())
}
