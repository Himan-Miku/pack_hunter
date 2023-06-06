mod functions;
mod structs;

use clap::Parser;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use functions::{javascript, rust};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use std::{error::Error, io};
use structs::{
    cli::{Cli, Commands},
    js_structs::{Package, ResponseObject, SearchResult, SingleResponseObject},
    rs_structs::{Crate, ResponseObj},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let mut list_results: Vec<(String, String, &str)> = Vec::new();

    match &cli.command {
        Commands::Javascript(js_options) => {
            let js_results = javascript::search_pack(js_options).await.unwrap();
            let ResponseObject { objects } = js_results;
            for object in objects {
                let SearchResult { package } = object;
                let Package { name, version, .. } = package;
                list_results.push((name, version, "js"));
            }
        }
        Commands::Rust(rs_options) => {
            let rust_results = rust::search_pack(rs_options).await.unwrap();
            let ResponseObj { crates } = rust_results;
            for _crate in crates {
                let Crate {
                    name, max_version, ..
                } = _crate;
                list_results.push((name, max_version.unwrap_or(String::from("1.0.0")), "rs"));
            }
        }
    }

    println!("{:?}", list_results);

    // Initialize the terminal
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();

    // Set up the interactive selection UI
    let mut list_state = ListState::default();
    let mut sel_index = 0;
    list_state.select(Some(sel_index));
    let mut selected_data: Option<SingleResponseObject> = None;

    loop {
        // Render the UI
        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .split(f.size());

                let items: Vec<ListItem> = list_results
                    .iter()
                    .map(|list_result| {
                        let item = ListItem::new(list_result.0.to_string()).style(Style::default());
                        if let Some(selected_index) = list_state.selected() {
                            if selected_index
                                == list_results
                                    .iter()
                                    .position(|x| *x == *list_result)
                                    .unwrap()
                            {
                                return item
                                    .style(Style::default().add_modifier(Modifier::REVERSED));
                            }
                        }
                        item
                    })
                    .collect();

                let items_list = List::new(items)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Choose a list_result"),
                    )
                    .style(Style::default())
                    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                    .highlight_symbol("> ");

                f.render_widget(items_list, chunks[0]);

                if let Some(data) = &selected_data {
                    let text = format!("{:#?}", data);
                    let block = Block::default().title("Fetched Data").borders(Borders::ALL);
                    let paragraph = Paragraph::new(text)
                        .style(Style::default().fg(Color::White))
                        .block(block);
                    f.render_widget(paragraph, chunks[0]); // Adjust the index if necessary
                }
            })
            .unwrap();

        // Handle keyboard events
        if let Ok(Event::Key(KeyEvent { code, kind, .. })) = crossterm::event::read() {
            match kind {
                KeyEventKind::Press => match code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Up => {
                        if sel_index > 0 {
                            sel_index -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if sel_index < list_results.len() {
                            sel_index += 1;
                        }
                    }
                    KeyCode::Enter => {
                        if let Some(selected_index) = list_state.selected() {
                            let selected_result = &list_results[selected_index];

                            if selected_result.2 == "js" {
                                let url = format!(
                                    "https://registry.npmjs.org/{}/{}",
                                    selected_result.0, selected_result.1
                                );
                                let client = reqwest::Client::new();
                                let mut headers = HeaderMap::new();
                                headers
                                    .insert(USER_AGENT, HeaderValue::from_static("himan-crawler"));
                                let res = client.get(url).headers(headers).send().await?;
                                let data: SingleResponseObject = res.json().await?;
                                selected_data = Some(data);
                            } else {
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }

            list_state.select(Some(sel_index));
        }
    }
    Ok(())
}
