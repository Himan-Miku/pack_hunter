mod functions;
mod structs;

use clap::Parser;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use functions::{javascript, rust};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use std::{error::Error, io};
use structs::{
    cli::{Cli, Commands},
    js_structs::{Package, Repo, ResponseObject, SearchResult, SingleResponseObject},
    rs_structs::{Crate, ResponseObj, SingleResponseObj},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Terminal,
};

use crate::structs::rs_structs::Version;

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
    let mut selected_data_rs: Option<SingleResponseObj> = None;

    loop {
        // Render the UI
        terminal
            .draw(|f| {
                let mut list_head: Option<&str> = None;

                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(35), Constraint::Percentage(65)].as_ref())
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
                        if list_results.get(0).unwrap().2 == "js" {
                            list_head = Some("Package");
                        } else {
                            list_head = Some("Crate");
                        }
                        item
                    })
                    .collect();

                let items_list = List::new(items)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                            .title(format!(" Choose a {} -> ", list_head.unwrap())),
                    )
                    .style(Style::default().fg(Color::White))
                    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                    .highlight_symbol("> ");

                f.render_widget(items_list, chunks[0]);

                if let Some(data) = &selected_data {
                    let SingleResponseObject {
                        name,
                        version,
                        description,
                        repository,
                        license,
                        homepage,
                    } = data;

                    let repo_url = repository
                        .as_ref()
                        .and_then(|repo| {
                            let Repo { url } = repo;
                            url.as_ref().map(|s| s.clone())
                        })
                        .unwrap_or(String::from(""));
                    let des_str = description
                        .as_ref()
                        .map(|s| s.clone())
                        .unwrap_or(String::from(""));
                    let lic_str = license
                        .as_ref()
                        .map(|s| s.clone())
                        .unwrap_or("".to_string());
                    let home_str = homepage
                        .as_ref()
                        .map(|s| s.clone())
                        .unwrap_or("".to_string());

                    let text = vec![
                        Spans::from(vec![Span::raw("\n"), Span::raw("\n"), Span::raw("\n")]),
                        Spans::from(vec![
                            Span::styled(
                                "Package Name : ",
                                Style::default()
                                    .fg(Color::White)
                                    .add_modifier(Modifier::BOLD),
                            ),
                            Span::styled(
                                name,
                                Style::default()
                                    .fg(Color::Green)
                                    .add_modifier(Modifier::BOLD),
                            ),
                        ]),
                        Spans::from(Span::raw("\n")),
                        Spans::from(vec![
                            Span::styled("Package Version : ", Style::default().fg(Color::White)),
                            Span::styled(version, Style::default().fg(Color::Cyan)),
                        ]),
                        Spans::from(vec![Span::raw("\n"), Span::raw("\n")]),
                        Spans::from(vec![
                            Span::styled(
                                "Package Description : ",
                                Style::default().fg(Color::White),
                            ),
                            Span::styled(des_str, Style::default().fg(Color::Yellow)),
                        ]),
                        Spans::from(vec![Span::raw("\n"), Span::raw("\n")]),
                        Spans::from(vec![
                            Span::styled("Repository URL : ", Style::default().fg(Color::White)),
                            Span::styled(repo_url, Style::default().fg(Color::Rgb(160, 32, 240))),
                        ]),
                        Spans::from(Span::raw("\n")),
                        Spans::from(vec![
                            Span::styled("Homepage URL : ", Style::default().fg(Color::White)),
                            Span::styled(home_str, Style::default().fg(Color::Rgb(160, 32, 240))),
                        ]),
                        Spans::from(vec![Span::raw("\n"), Span::raw("\n")]),
                        Spans::from(vec![
                            Span::styled("License : ", Style::default().fg(Color::White)),
                            Span::styled(lic_str, Style::default().fg(Color::LightRed)),
                        ]),
                    ];
                    let block = Block::default()
                        .title(" Package Info -> ")
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded);
                    let paragraph = Paragraph::new(text)
                        .style(Style::default().fg(Color::White))
                        .block(block)
                        .alignment(Alignment::Left)
                        .wrap(Wrap { trim: true });
                    f.render_widget(paragraph, chunks[1]);
                }

                if let Some(data) = &selected_data_rs {
                    let SingleResponseObj {
                        version:
                            Version {
                                crate_name,
                                downloads,
                                license,
                                readme_path,
                                num,
                                ..
                            },
                    } = data;
                    let text = vec![
                        Spans::from(vec![Span::raw("\n"), Span::raw("\n"), Span::raw("\n")]),
                        Spans::from(vec![
                            Span::styled(
                                "Crate Name : ",
                                Style::default()
                                    .fg(Color::White)
                                    .add_modifier(Modifier::BOLD),
                            ),
                            Span::styled(
                                crate_name,
                                Style::default()
                                    .fg(Color::Green)
                                    .add_modifier(Modifier::BOLD),
                            ),
                        ]),
                        Spans::from(Span::raw("\n")),
                        Spans::from(vec![
                            Span::styled("Crate Version : ", Style::default().fg(Color::White)),
                            Span::styled(num, Style::default().fg(Color::Cyan)),
                        ]),
                        Spans::from(Span::raw("\n")),
                        Spans::from(vec![
                            Span::styled("Crate Downloads : ", Style::default().fg(Color::White)),
                            Span::styled(downloads.to_string(), Style::default().fg(Color::Cyan)),
                        ]),
                        Spans::from(vec![Span::raw("\n"), Span::raw("\n")]),
                        Spans::from(vec![
                            Span::styled("README : ", Style::default().fg(Color::White)),
                            Span::styled(
                                format!("https://crates.io{}", readme_path),
                                Style::default().fg(Color::Rgb(160, 32, 240)),
                            ),
                        ]),
                        Spans::from(vec![Span::raw("\n"), Span::raw("\n")]),
                        Spans::from(vec![
                            Span::styled("License : ", Style::default().fg(Color::White)),
                            Span::styled(license, Style::default().fg(Color::LightRed)),
                        ]),
                    ];
                    let block = Block::default()
                        .title(" Crate Info -> ")
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded);
                    let paragraph = Paragraph::new(text)
                        .style(Style::default().fg(Color::White))
                        .block(block)
                        .alignment(Alignment::Left)
                        .wrap(Wrap { trim: true });
                    f.render_widget(paragraph, chunks[1]);
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
                                // TODO : add rust crate info fucntionalty
                                let url = format!(
                                    "https://crates.io/api/v1/crates/{}/{}",
                                    selected_result.0, selected_result.1
                                );
                                let client = reqwest::Client::new();
                                let mut headers = HeaderMap::new();
                                headers
                                    .insert(USER_AGENT, HeaderValue::from_static("himan-crawler"));
                                let res = client.get(url).headers(headers).send().await?;
                                let data: SingleResponseObj = res.json().await?;
                                selected_data_rs = Some(data);
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
