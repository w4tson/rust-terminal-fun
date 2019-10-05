pub mod model;
use std::fs;
use chrono::{DateTime, Utc};
use crate::devoxx::model::{Talk, ScheduleItem};

pub fn get_talks_by_day(day: &String) -> Result<Vec<ScheduleItem>, failure::Error> {
    let contents = fs::read_to_string(format!("devoxx-data/{}.json", day))?;
    serde_json::from_str(&contents)
        .map_err(failure::Error::from)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let talks = get_talks_by_day(&"monday".to_string());
        assert_eq!(true, talks.is_ok());
        if let Ok(talks) = talks {
            assert_eq!(talks.len(), 38);
        }
    }
}
