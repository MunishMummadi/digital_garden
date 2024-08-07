use git2::{Repository, RemoteCallbacks, Cred};
use std::path::Path;
use crate::storage::garden_path;

pub fn sync_notes() -> Result<(), Box<dyn std::error::Error>> {
    let repo_path = garden_path();

    // Initialize the repository if it doesn't exist
    if !Path::new(&repo_path).join(".git").exists() {
        Repository::init(&repo_path)?;
        println!("Initialized new Git repository in the digital garden.");
    }

    let repo = Repository::open(&repo_path)?;

    // Add all files to the index
    let mut index = repo.index()?;
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;

    // Create a commit
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    let signature = repo.signature()?;
    let parent_commit = repo.head().ok().and_then(|h| h.target()).and_then(|oid| repo.find_commit(oid).ok());
    let parents = parent_commit.as_ref().map(|c| vec![c]).unwrap_or(vec![]);

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Sync digital garden",
        &tree,
        parents.as_slice(),
    )?;

    println!("Changes committed locally.");

    // Push to remote (assuming 'origin' and 'main' branch)
    // Note: This part requires proper authentication setup
    let mut remote = repo.find_remote("origin")?;
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, _username_from_url, _allowed_types| {
        // This is a simple example. In a real application, you'd want to handle this more securely.
        Cred::ssh_key(
            "git",
            None,
            std::path::Path::new(&format!("{}/.ssh/id_rsa", std::env::var("HOME").unwrap_or_default())),
            None,
        )
    });

    remote.push(&["refs/heads/main:refs/heads/main"], Some(git2::PushOptions::new().remote_callbacks(callbacks)))?;

    println!("Changes pushed to remote repository.");

    Ok(())
}