use tui::style::{Color, Style};
use crate::devoxx::model::ScheduleItem;
use chrono::Weekday;

#[derive(PartialEq)]
pub enum Mode {
    NORMAL,
    FILTERED,
    SEARCH
}


#[allow(dead_code)]
pub struct App {
    pub day: Weekday,
    pub talks: Vec<ScheduleItem>,
    pub selected: Option<usize>,
    pub search_text : String,
    pub mode: Mode,
    pub info_style: Style,
    pub warning_style: Style,
    pub error_style: Style,
    pub critical_style: Style,
}

impl App {
    pub fn new(talks: Vec<ScheduleItem>) -> App {
        App {
            day : Weekday::Mon,
            talks,
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

    pub fn get_selected(&self) -> Option<&ScheduleItem> {
        self.selected.map(|i |{
            self.talks.get(i).unwrap()
        })
    }
    
    pub fn talks(&self) -> Vec<&ScheduleItem> {
        self.talks
            .iter()
            .filter(|&repo| self.filter(repo))
            .collect()
    }

    pub fn talk_titles(&self) -> Vec<&str> {
        self.talks()
            .iter()
            .map(|talk| talk.get_title())
            .collect()
    }
    
    fn filter(&self, repo: &ScheduleItem) -> bool {
       if self.search_text.is_empty() {
           true
       } else {
           repo.get_title().to_lowercase().contains(&self.search_text.to_lowercase())
       }
    }
}