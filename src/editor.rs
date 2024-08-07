use std::io::stdout;
use crossterm::{
    execute,
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction},
    style::{Style, Color},
    text::{Spans, Span},
    Terminal,
};
use crate::storage::{load_note, save_note};

pub fn edit_note(title: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut note = load_note(title)?;
    let mut cursor_position = note.content.len();

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(1),
                        Constraint::Length(1),
                    ].as_ref()
                )
                .split(f.size());

            let title_block = Block::default()
                .title("Title")
                .borders(Borders::ALL);
            let title_text = Paragraph::new(note.title.as_ref())
                .block(title_block);
            f.render_widget(title_text, chunks[0]);

            let content_block = Block::default()
                .title("Content")
                .borders(Borders::ALL);
            let content = &note.content;
            let cursor_line = content[..cursor_position].matches('\n').count();
            let cursor_column = cursor_position - content[..cursor_position].rfind('\n').map_or(0, |i| i + 1);
            let content_text = Paragraph::new(content.as_str())
                .block(content_block)
                .scroll((cursor_line as u16, cursor_column as u16));
            f.render_widget(content_text, chunks[1]);

            let instructions = Spans::from(vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().fg(Color::Red)),
                Span::raw(" to save and quit, "),
                Span::styled("Ctrl+C", Style::default().fg(Color::Red)),
                Span::raw(" to quit without saving"),
            ]);
            let instructions_paragraph = Paragraph::new(instructions);
            f.render_widget(instructions_paragraph, chunks[2]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => {
                    save_note(&note)?;
                    break;
                }
                KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
                    break;
                }
                KeyCode::Char(c) => {
                    note.content.insert(cursor_position, c);
                    cursor_position += 1;
                    note.update_content(note.content.clone());
                }
                KeyCode::Backspace => {
                    if cursor_position > 0 {
                        note.content.remove(cursor_position - 1);
                        cursor_position -= 1;
                        note.update_content(note.content.clone());
                    }
                }
                KeyCode::Delete => {
                    if cursor_position < note.content.len() {
                        note.content.remove(cursor_position);
                        note.update_content(note.content.clone());
                    }
                }
                KeyCode::Left => {
                    if cursor_position > 0 {
                        cursor_position -= 1;
                    }
                }
                KeyCode::Right => {
                    if cursor_position < note.content.len() {
                        cursor_position += 1;
                    }
                }
                KeyCode::Up => {
                    let current_line_start = note.content[..cursor_position].rfind('\n').map_or(0, |i| i + 1);
                    let previous_line_start = note.content[..current_line_start.saturating_sub(1)]
                        .rfind('\n')
                        .map_or(0, |i| i + 1);
                    let column = cursor_position - current_line_start;
                    cursor_position = previous_line_start + column.min(current_line_start - previous_line_start - 1);
                }
                KeyCode::Down => {
                    let current_line_start = note.content[..cursor_position].rfind('\n').map_or(0, |i| i + 1);
                    if let Some(next_line_start) = note.content[cursor_position..].find('\n') {
                        let next_line_start = cursor_position + next_line_start + 1;
                        let column = cursor_position - current_line_start;
                        cursor_position = next_line_start + column.min(
                            note.content[next_line_start..]
                                .find('\n')
                                .map_or(note.content.len() - next_line_start, |i| i)
                        );
                    }
                }
                KeyCode::Enter => {
                    note.content.insert(cursor_position, '\n');
                    cursor_position += 1;
                    note.update_content(note.content.clone());
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    println!("Note '{}' updated successfully.", title);

    Ok(())
}