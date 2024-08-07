use std::fs;
use std::path::{Path, PathBuf};
use crate::note::Note;

const GARDEN_PATH: &str = "digital_garden";

pub fn garden_path() -> PathBuf {
    Path::new(GARDEN_PATH).to_path_buf()
}

fn ensure_garden_exists() -> std::io::Result<()> {
    fs::create_dir_all(garden_path())
}

fn note_path(title: &str) -> PathBuf {
    garden_path().join(format!("{}.json", title))
}

pub fn create_note(title: &str) -> Result<(), Box<dyn std::error::Error>> {
    ensure_garden_exists()?;
    let path = note_path(title);
    if path.exists() {
        println!("Note with this title already exists.");
        return Ok(());
    }
    let note = Note::new(title.to_string());
    let note_json = serde_json::to_string_pretty(&note)?;
    fs::write(path, note_json)?;
    println!("Note '{}' created successfully.", title);
    Ok(())
}

pub fn list_notes() -> Result<(), Box<dyn std::error::Error>> {
    ensure_garden_exists()?;
    for entry in fs::read_dir(garden_path())? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();
        if file_name.ends_with(".json") {
            println!("{}", file_name.trim_end_matches(".json"));
        }
    }
    Ok(())
}

pub fn view_note(title: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = note_path(title);
    if !path.exists() {
        println!("Note not found.");
        return Ok(());
    }
    let note_content = fs::read_to_string(path)?;
    let note: Note = serde_json::from_str(&note_content)?;
    println!("Title: {}", note.title);
    println!("Content: {}", note.content);
    println!("Created at: {}", note.created_at);
    println!("Updated at: {}", note.updated_at);
    println!("Links: {:?}", note.links);
    println!("Tags: {:?}", note.tags);
    println!("Version: {}", note.version);
    Ok(())
}

pub fn link_notes(from: &str, to: &str) -> Result<(), Box<dyn std::error::Error>> {
    let from_path = note_path(from);
    let to_path = note_path(to);

    if !from_path.exists() || !to_path.exists() {
        println!("One or both notes not found.");
        return Ok(());
    }

    let mut from_note: Note = serde_json::from_str(&fs::read_to_string(&from_path)?)?;
    let mut to_note: Note = serde_json::from_str(&fs::read_to_string(&to_path)?)?;

    from_note.add_link(to.to_string());
    to_note.add_link(from.to_string());

    fs::write(from_path, serde_json::to_string_pretty(&from_note)?)?;
    fs::write(to_path, serde_json::to_string_pretty(&to_note)?)?;

    println!("Link created between '{}' and '{}'.", from, to);
    Ok(())
}

pub fn add_tag(title: &str, tag: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = note_path(title);
    if !path.exists() {
        println!("Note not found.");
        return Ok(());
    }
    let mut note: Note = serde_json::from_str(&fs::read_to_string(&path)?)?;
    note.add_tag(tag.to_string());
    fs::write(path, serde_json::to_string_pretty(&note)?)?;
    println!("Tag '{}' added to note '{}'.", tag, title);
    Ok(())
}

pub fn save_note(note: &Note) -> Result<(), Box<dyn std::error::Error>> {
    let path = note_path(&note.title);
    fs::write(path, serde_json::to_string_pretty(&note)?)?;
    Ok(())
}

pub fn load_note(title: &str) -> Result<Note, Box<dyn std::error::Error>> {
    let path = note_path(title);
    let note_content = fs::read_to_string(path)?;
    let note: Note = serde_json::from_str(&note_content)?;
    Ok(note)
}