mod functions;
mod structs;

use clap::Parser;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use functions::{javascript, rust};
use std::io;
use structs::cli::{Cli, Commands};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};

enum ResponseType {
    ResponseObject,
    ResponseObj,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let list_results: Vec<ResponseType> = Vec::new();

    match &cli.command {
        Commands::Javascript(js_options) => {
            let js_results = javascript::search_pack(js_options).await.unwrap();
        }
        Commands::Rust(rs_options) => {
            let rust_results = rust::search_pack(rs_options).await.unwrap();
        }
    }

    // let frameworks = ["React", "Vue", "Angular"];

    // // Initialize the terminal
    // let stdout = io::stdout();
    // let backend = CrosstermBackend::new(stdout);
    // let mut terminal = Terminal::new(backend).unwrap();
    // terminal.clear().unwrap();

    // // Set up the interactive selection UI
    // let mut list_state = ListState::default();
    // let mut sel_index = 0;
    // list_state.select(Some(sel_index));

    // loop {
    //     // Render the UI
    //     terminal
    //         .draw(|f| {
    //             let chunks = Layout::default()
    //                 .constraints([Constraint::Percentage(100)].as_ref())
    //                 .split(f.size());

    //             let items: Vec<ListItem> = frameworks
    //                 .iter()
    //                 .map(|framework| {
    //                     let item = ListItem::new(framework.to_string()).style(Style::default());
    //                     if let Some(selected_index) = list_state.selected() {
    //                         if selected_index
    //                             == frameworks.iter().position(|x| *x == *framework).unwrap()
    //                         {
    //                             return item
    //                                 .style(Style::default().add_modifier(Modifier::REVERSED));
    //                         }
    //                     }
    //                     item
    //                 })
    //                 .collect();

    //             let items_list = List::new(items)
    //                 .block(
    //                     Block::default()
    //                         .borders(Borders::ALL)
    //                         .title("Choose a framework"),
    //                 )
    //                 .style(Style::default())
    //                 .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    //                 .highlight_symbol("> ");

    //             f.render_widget(items_list, chunks[0]);
    //         })
    //         .unwrap();

    //     // Handle keyboard events
    //     if let Ok(Event::Key(KeyEvent { code, kind, .. })) = crossterm::event::read() {
    //         match kind {
    //             KeyEventKind::Press => match code {
    //                 KeyCode::Char('q') => {
    //                     break;
    //                 }
    //                 KeyCode::Up => {
    //                     if sel_index > 0 {
    //                         sel_index -= 1;
    //                     }
    //                 }
    //                 KeyCode::Down => {
    //                     if sel_index < frameworks.len() {
    //                         sel_index += 1;
    //                     }
    //                 }
    //                 KeyCode::Enter => {
    //                     if let Some(selected_index) = list_state.selected() {
    //                         println!("Selected framework: {}", frameworks[selected_index]);
    //                         break;
    //                     }
    //                 }
    //                 _ => {}
    //             },
    //             _ => {}
    //         }

    //         list_state.select(Some(sel_index));
    //     }
    // }
}
