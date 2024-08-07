use structopt::StructOpt;
use crate::{storage, search, editor, visualizer, sync};

#[derive(StructOpt, Debug)]
#[structopt(name = "digital-garden", about = "A CLI tool for managing your digital garden")]
pub enum Cli {
    #[structopt(name = "new", about = "Create a new note")]
    New { title: String },

    #[structopt(name = "list", about = "List all notes")]
    List,

    #[structopt(name = "view", about = "View a specific note")]
    View { title: String },

    #[structopt(name = "edit", about = "Edit a note")]
    Edit { title: String },

    #[structopt(name = "link", about = "Create a link between two notes")]
    Link { from: String, to: String },

    #[structopt(name = "tag", about = "Add a tag to a note")]
    Tag { title: String, tag: String },

    #[structopt(name = "search", about = "Search notes")]
    Search { query: String },

    #[structopt(name = "visualize", about = "Visualize note connections")]
    Visualize,

    #[structopt(name = "sync", about = "Sync notes with Git repository")]
    Sync,
}

impl Cli {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Cli::New { title } => storage::create_note(title),
            Cli::List => storage::list_notes(),
            Cli::View { title } => storage::view_note(title),
            Cli::Edit { title } => editor::edit_note(title),
            Cli::Link { from, to } => storage::link_notes(from, to),
            Cli::Tag { title, tag } => storage::add_tag(title, tag),
            Cli::Search { query } => search::search_notes(query),
            Cli::Visualize => visualizer::visualize_notes(),
            Cli::Sync => sync::sync_notes(),
        }
    }
}