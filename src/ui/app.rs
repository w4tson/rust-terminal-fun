use tui::style::{Color, Style};
use crate::devoxx::model::ScheduleItem;
use chrono::Weekday;
use crate::devoxx::{get_talks_by_weekday};
use chrono::Weekday::Mon;

#[derive(PartialEq)]
pub enum Mode {
    Normal,
    Filtered,
    Search
}

#[allow(dead_code)]
pub struct App {
    pub day: Weekday,
    pub talks: Vec<ScheduleItem>,
    pub selected: Option<usize>,
    pub search_text : String,
    pub mode: Mode
}

impl App {
    pub fn new() -> Result<App, failure::Error> {
        let talks = get_talks_by_weekday(&Mon)?;

        Ok(App {
            day : Weekday::Mon,
            talks,
            search_text: String::new(),
            selected: Some(0),
            mode : Mode::Normal
        })
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
    
    pub fn next_tab(&mut self) {
        let new_day = if self.day == Weekday::Fri { Weekday::Mon } else { self.day.succ() };
        self.set_current_day(new_day);
    }

    pub fn previous_tab(&mut self) {
        let new_day = if self.day == Weekday::Mon { Weekday::Fri } else { self.day.pred() };
        self.set_current_day(new_day);
    }
    
    fn set_current_day(&mut self, day: Weekday) {
        self.day = day;
        self.talks = get_talks_by_weekday(&self.day)
            .expect("Talks not found");
        self.selected = Some(0);
    }

    pub fn next_talk(&mut self) {
        self.selected = if let Some(selected) = self.selected {
            if selected >= self.talks().len() - 1 {
                Some(0)
            } else {
                Some(selected + 1)
            }
        } else {
            Some(0)
        }
    }
    
    pub fn previous_talk(&mut self) {
        self.selected = if let Some(selected) = self.selected {
            if selected > 0 {
                Some(selected - 1)
            } else {
                Some(self.talks().len() - 1)
            }
        } else {
            Some(0)
        }
    }
}