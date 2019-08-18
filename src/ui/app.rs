use crate::github::{Repo};
use tui::style::{Color, Style};

#[derive(PartialEq)]
pub enum Mode {
    NORMAL,
    SEARCH
}

#[allow(dead_code)]
pub struct App {
    pub repos: Vec<Repo>,
    pub selected: Option<usize>,
    pub search_text : String,
    pub mode: Mode,
    pub info_style: Style,
    pub warning_style: Style,
    pub error_style: Style,
    pub critical_style: Style,
}

impl App {
    pub fn new(repos: Vec<Repo>) -> App {
        App {
            repos,
            search_text: String::new(),
            selected: Some(0),
            mode : Mode::NORMAL,
            info_style: Style::default().fg(Color::White),
            warning_style: Style::default().fg(Color::Yellow),
            error_style: Style::default().fg(Color::Magenta),
            critical_style: Style::default().fg(Color::Red),
        }
    }

    pub fn advance(&mut self) {
        //tick
    }

    pub fn get_selected(&self) -> Option<&Repo> {
        self.selected.map(|i |{
            self.repos.get(i).unwrap()
        })
    }
    
    pub fn repos(&self) -> Vec<&Repo> {
        self.repos
            .iter()
            .filter(|&repo| self.filter(repo))
            .collect()
    }

    pub fn repo_names(&self) -> Vec<&str> {
        self.repos()
            .iter()
            .map(|repo| repo.name.as_str())
            .collect()
    }
    
    fn filter(&self, repo: &Repo) -> bool {
       if self.search_text.is_empty() {
           true
       } else {
           repo.full_name.contains(&self.search_text)
       }
    }
}