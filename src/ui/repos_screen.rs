use super::util;
use std::io;

use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout, Alignment};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, SelectableList, Text, Widget, Paragraph, Tabs};
use tui::Terminal;

use util::event::{Event, Events};
use super::app::App;
use std::io::Write;
use termion::cursor::Goto;
use crate::ui::app::Mode;
use crate::devoxx::get_talks_by_day;
use chrono::Weekday;


pub fn run() -> Result<(), failure::Error> {
    let talks = get_talks_by_day(&"monday".to_string())?;
    
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();

    // App
    let mut app = App::new(talks);

    loop {
        terminal.draw(|mut f| {
            let main = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(20), Constraint::Length(1)].as_ref())
                .split(f.size());

            Tabs::default()
                .block(Block::default().borders(Borders::ALL).title("Day"))
                .titles(&["Monday","Tuesday","Wednesday","Thursday", "Friday"])
                .select(app.day.num_days_from_monday() as usize)
                .style(Style::default().fg(Color::Cyan))
                .highlight_style(Style::default().fg(Color::Yellow))
                .render(&mut f, main[0]);
            
            if app.mode == Mode::SEARCH {
                Paragraph::new([Text::raw(format!("/{}", &app.search_text))].iter())
                    .style(Style::default().fg(Color::Yellow))
                    .render(&mut f, main[2]);
            }
            
            
            let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(main[1]);

           

            let style = Style::default().fg(Color::White).bg(Color::Black);
            SelectableList::default()
                .block(Block::default().borders(Borders::ALL).title("Talks"))
                .items(&app.talk_titles())
                .select(app.selected)
                .style(style)
                .highlight_style(style.bg(Color::LightGreen).fg(Color::White).modifier(Modifier::BOLD))
                .highlight_symbol(">")
                .render(&mut f, chunks[0]);
            

            if let Some(talk) = app.get_selected() {
                    
                let text = vec![
                    Text::styled(format!("Title: {}\n", talk.get_title()), Style::default().fg(Color::Yellow)),
                    Text::raw(format!("Room : {}\n", talk.room_name)),
                    Text::raw(format!("From : {}\n", talk.from_date.to_rfc2822())),
                    Text::raw(format!("To   : {}\n", talk.to_date.to_rfc2822())),
                ];
                
                Paragraph::new(text.iter())
                    .block(Block::default().title("Talk Details").borders(Borders::ALL))
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
                },
                Key::Char('\t') => {
                    app.day = if app.day == Weekday::Fri { Weekday::Mon } else { app.day.succ() };
                    
                },
                Key::Left => {
                    app.selected = None;
                }
                Key::Down => {
                    app.selected = if let Some(selected) = app.selected {
                        if selected >= app.talks().len() - 1 {
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
                            Some(app.talks().len() - 1)
                        }
                    } else {
                        Some(0)
                    }
                }
                Key::Char('\n') if app.mode == Mode::NORMAL => {
                    if let Some(selected) = app.selected {
                        if let Some(_talk) = app.talks.get(selected) {
//                          pressed enter on a talk
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