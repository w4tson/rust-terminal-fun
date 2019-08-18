use super::util;
use std::io;

use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout, Alignment};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, SelectableList, Text, Widget, Paragraph};
use tui::Terminal;

use util::event::{Event, Events};
use crate::github::{get_repos};
use super::app::App;
use std::io::Write;
use termion::cursor::Goto;
use crate::ui::app::Mode;


pub fn run() -> Result<(), failure::Error> {
    let repos = get_repos("w4tson")?;
    
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();

    // App
    let mut app = App::new(repos);

    loop {
        terminal.draw(|mut f| {
            let main = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(20), Constraint::Length(1)].as_ref())
                .split(f.size());

            if app.mode == Mode::SEARCH {
                Paragraph::new([Text::raw(format!("/{}", &app.search_text))].iter())
                    .style(Style::default().fg(Color::Yellow))
                    .render(&mut f, main[1]);
            }
            
                let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(main[0]);

            let style = Style::default().fg(Color::White).bg(Color::Black);
            SelectableList::default()
                .block(Block::default().borders(Borders::ALL).title("Repository"))
                .items(&app.repo_names())
                .select(app.selected)
                .style(style)
                .highlight_style(style.bg(Color::LightGreen).fg(Color::White).modifier(Modifier::BOLD))
                .highlight_symbol(">")
                .render(&mut f, chunks[0]);
            

            if let Some(repo) = app.get_selected() {
                    
                let text = vec![
                    Text::styled(format!("Full Name: {}\n", repo.full_name), Style::default().fg(Color::Red)),
                    Text::raw(format!("Owner: {}\n", repo.owner.login)),
                    Text::raw(format!("Language: {}\n", repo.language.as_ref().unwrap_or(&("".to_string())))),
                    Text::raw(format!("Created: {}\n", repo.created_at.to_rfc2822())),
                ];
                
                Paragraph::new(text.iter())
                    .block(Block::default().title("Repository Details").borders(Borders::ALL))
                    .style(Style::default().fg(Color::White).bg(Color::Black))
                    .alignment(Alignment::Left)
                    .wrap(true)
                    .render(&mut f, chunks[1]);
            }
            
        })?;

        if let Ok(rect) = terminal.size() {
            write!(
                terminal.backend_mut(),
                "{}",
                Goto((app.search_text.len() +2 ) as u16, rect.height  as u16)
            )?;
        }
        
        // stdout is buffered, flush it to see the effect immediately when hitting backspace.
        io::stdout().flush().ok();

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Left => {
                    app.selected = None;
                }
                Key::Down => {
                    app.selected = if let Some(selected) = app.selected {
                        if selected >= app.repos().len() - 1 {
                            Some(0)
                        } else {
                            Some(selected + 1)
                        }
                    } else {
                        Some(0)
                    }
                }
                Key::Up => {
                    app.selected = if let Some(selected) = app.selected {
                        if selected > 0 {
                            Some(selected - 1)
                        } else {
                            Some(app.repos().len() - 1)
                        }
                    } else {
                        Some(0)
                    }
                }
                Key::Char('\n') if app.mode == Mode::NORMAL => {
                    if let Some(selected) = app.selected {
                        if let Some(_repo) = app.repos.get(selected) {
//                          pressed enter on a repo
                        }
                    }
                }
                Key::Char('\n') if app.mode == Mode::SEARCH => {
                    app.mode == Mode::NORMAL;
                    terminal.hide_cursor()?;
                }
                Key::Char('/') => {
                    app.mode = Mode::SEARCH;
                    terminal.show_cursor()?;
                }
                Key::Esc => {
                    terminal.hide_cursor()?;
                    app.search_text = "".to_string();
                    app.mode = Mode::NORMAL
                },
                Key::Backspace if app.mode == Mode::SEARCH => {
                    app.search_text.pop();
                }
                Key::Char(c) if app.mode == Mode::SEARCH => {
                    app.search_text.push(c);
                }
                _ => {}
            },
            Event::Tick => {
                app.advance();
            }
        }
    }

    Ok(())
}