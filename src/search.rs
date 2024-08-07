use tantivy::schema::{Schema, STORED, TEXT};
use tantivy::{doc, Index, ReloadPolicy};
use tantivy::query::QueryParser;
use std::fs;
use crate::note::Note;
use crate::storage::{garden_path, load_note};

pub fn search_notes(query: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("content", TEXT);
    let schema = schema_builder.build();

    let index_path = garden_path().join("search_index");
    fs::create_dir_all(&index_path)?;
    let index = Index::create_in_dir(&index_path, schema.clone())?;

    let mut index_writer = index.writer(50_000_000)?;

    for entry in fs::read_dir(garden_path())? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let note: Note = load_note(path.file_stem().unwrap().to_str().unwrap())?;
            index_writer.add_document(doc!(
                schema.get_field("title").unwrap() => note.title,
                schema.get_field("content").unwrap() => note.content,
            ))?;
        }
    }

    index_writer.commit()?;

    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()?;

    let searcher = reader.searcher();

    let query_parser = QueryParser::for_index(&index, vec![schema.get_field("title").unwrap(), schema.get_field("content").unwrap()]);

    let query = query_parser.parse_query(query)?;
    let top_docs = searcher.search(&query, &tantivy::collector::TopDocs::with_limit(10))?;

    println!("Search results:");
    for (_score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address)?;
        let title = retrieved_doc.get_first(schema.get_field("title").unwrap()).unwrap().as_text().unwrap();
        println!("- {}", title);
    }

    Ok(())
}