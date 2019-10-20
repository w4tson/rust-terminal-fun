pub mod model;
use std::fs;
use crate::devoxx::model::{ScheduleItem};
use chrono::{Weekday, DateTime, Utc};
use std::str::FromStr;

const devoxx_host: &str = "dvbe19.cfp.dev";

pub fn get_talks_by_weekday(day: &Weekday) -> Result<Vec<ScheduleItem>, failure::Error> {
    let day = match day {
        Weekday::Mon => "monday",
        Weekday::Tue => "tuesday",
        Weekday::Wed => "wednesday",
        Weekday::Thu => "thursday",
        Weekday::Fri => "friday",
        _ => "monday"
    };

    get_talks_by_day_api(day)
}

pub fn get_talks_by_day(day: &str) -> Result<Vec<ScheduleItem>, failure::Error> {
    let contents = fs::read_to_string(format!("devoxx-data/{}.json", day))?;
    serde_json::from_str(&contents)
        .map_err(failure::Error::from)
}

 pub fn get_talks_by_day_api(day: &str) -> Result<Vec<ScheduleItem>, failure::Error> {
    let request_url = format!("https://{}/api/public/schedules/{day}", devoxx_host, day = day);
    let talks: Vec<ScheduleItem> = reqwest::get(&request_url)?.json()?;
    Ok(talks)
}

#[derive(Debug)]
pub struct ScheduleItem2 {
    talk_title: Option<String>,
    from_date: DateTime<Utc>
}

pub fn get_schedule_items() -> Result<Vec<ScheduleItem2>, failure::Error> {
    Ok(read_talks()?.iter().map(to_schedule).collect())
}

fn to_schedule(line :&String) -> ScheduleItem2 {
    let tokens = line.split(",").collect::<Vec<&str>>();
    let talk_title = tokens.get(0).unwrap().to_string();
    let from_date = tokens.get(1).unwrap();
    let from_date = DateTime::from_str(from_date).expect(&format!("{} should include a date", line));
    ScheduleItem2 { talk_title: Some(talk_title), from_date }
}

pub fn read_talks() -> Result<Vec<String>, failure::Error> {
    std::fs::read_to_string("./devoxx-data/talks.txt")
        .map(|content| content.lines().map(|s| s.to_string()).collect())
        .map_err(failure::Error::from)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let talks = get_talks_by_day(&"monday".to_string());
        assert!(talks.is_ok());
        if let Ok(talks) = talks {
            assert_eq!(talks.len(), 38);
        }
    }
    
    #[test]
    fn test_read_talks() {
        assert!(read_talks().is_ok()); 
        if let Ok(talks) = read_talks() {
            assert_eq!(talks.len(), 12);
            assert_eq!(&talks.get(0).unwrap()[..12], "Monty Python");
        }
    }

    #[test]
    fn test_structured_data() {
        let items = get_schedule_items();
        assert!(items.is_ok());
        if let Ok(items) = items {
            assert_eq!(items.len(), 12);
            assert_eq!(items.get(0).unwrap().talk_title, Some("Monty Python meets the Cloud of Doom".to_string()));
        }
    }
    
    #[test]
    fn test_get_talks() {
        let mon_talks = get_talks_by_day("monday");
        assert!(mon_talks.is_ok());
        let mut found_rust_lab = false;
        let expected_title = Some(String::from("Rust for Java Developers"));
        if let Ok(talks) = mon_talks {
          
            for talk in &talks {
                match talk {
                    ScheduleItem { talk_title: title, .. } if title == &expected_title => found_rust_lab = true,
                    _ => ()
                } ;
            }

        }
        assert_eq!(found_rust_lab, true);
    }
    

}
