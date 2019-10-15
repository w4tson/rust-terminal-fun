use chrono::{DateTime, Utc};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Speaker {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub twitter: String,
    pub company: String
}

#[derive(Deserialize, Debug)]
pub struct Tag {
    pub name: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Talk {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub speakers: Vec<Speaker>,
    pub tags: Vec<Tag>,
    pub track_id: u32,
    pub track_name: String,
    pub session_type_name: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleItem {
    pub talk_title: Option<String>,
    pub talk_description: Option<String>,
    pub tags: Option<Vec<Tag>>,
    pub room_name: String,
    pub from_date: DateTime<Utc>,
    pub to_date: DateTime<Utc>,
    pub session_type_name: Option<String>
}

impl ScheduleItem {
    
    pub fn get_title(&self) -> &str {
        match &self.talk_title {
            Some(title) => &title,
            _ => match &self.session_type_name {
                Some(name) => &name,
                _ => ""
            }
        }
    }
    
    pub fn tags(&self) -> String {
        self.tags.as_ref()
            .map(|tags| tags.iter()
            .map(|tag| tag.name.as_str())
            .collect::<Vec<&str>>()
            .join(", "))
            .unwrap_or(String::new())
    }
}

