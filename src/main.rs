use std::{
    env::{self, args},
    fs::read_dir,
    io::{self, stdout},
    path::{Path, PathBuf},
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

struct AppState {
    current_index: usize,
    items: Vec<String>,
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
    // Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut state = AppState {
        current_index: 0,
        items: vec![
            "src/".to_string(),
            "Cargo.toml".to_string(),
            "README.md".to_string(),
            "docs/".to_string(),
        ],
    };

    loop {
        // Draw the UI
        terminal.draw(|f| ui(f, &state))?;

        // Poll for keyboard events (blocking until an event occurs)
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break, // Quit on 'q'
                KeyCode::Down | KeyCode::Char('j') => {
                    if state.current_index < state.items.len() - 1 {
                        state.current_index += 1;
                    }
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if state.current_index > 0 {
                        state.current_index -= 1;
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn ui(f: &mut Frame, state: &AppState) {
    // Split screen vertically: Top for header, Middle for main view, Bottom for status bar
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(1),    // Explorer Body
            Constraint::Length(3), // Status Bar / Keyhints
        ])
        .split(f.size());

    // 1. Header Widget
    let header = Paragraph::new(" Rust Filesystem Explorer — TUI Mode ")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        );
    f.render_widget(header, chunks[0]);

    // 2. Main Explorer List
    let list_items: Vec<ListItem> = state
        .items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == state.current_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD | Modifier::REVERSED)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(item.as_str()).style(style)
        })
        .collect();

    let file_list = List::new(list_items).block(
        Block::default()
            .title(" Directory Contents ")
            .borders(Borders::ALL),
    );
    f.render_widget(file_list, chunks[1]);

    // 3. Status Bar Widget
    let footer = Paragraph::new(" [j/k] Navigate | [Enter] Open | [q] Quit ")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}

// fn main() -> io::Result<()> {
//     let args: Vec<String> = args().collect();

//     let target_path = if args.len() > 1 {
//         PathBuf::from(&args[1])
//     } else {
//         env::current_dir()?
//     };

//     list_dir(&target_path)?;

//     cli(target_path.clone())?;

//     Ok(())
// }

// fn list_dir(path: &Path) -> io::Result<()> {
//     let entries = read_dir(path)?;

//     let mut item_list = Vec::new();

//     for entry in entries {
//         let entry = entry?;
//         item_list.push(entry);
//     }

//     item_list.sort_by(|a, b| {
//         let a_is_dir = a.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
//         let b_is_dir = b.file_type().map(|ft| ft.is_dir()).unwrap_or(false);

//         b_is_dir
//             .cmp(&a_is_dir)
//             .then_with(|| a.file_name().cmp(&b.file_name()))
//     });

//     for entry in item_list {
//         let metadata = entry.metadata()?;
//         let file_name = entry.file_name();
//         let filename_str = file_name.to_string_lossy();

//         let file_type = if metadata.is_dir() { "DIR" } else { "FILE" };

//         println!("{:<30} {:<10}", filename_str, file_type);
//     }
//     Ok(())
// }
