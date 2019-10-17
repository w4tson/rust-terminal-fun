use chrono::{DateTime, Utc};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Speaker {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub company: Option<String>
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
    pub session_type_name: Option<String>,
    pub speakers: Option<Vec<Speaker>>,
    pub timezone: String
}

impl ScheduleItem {

    pub fn local_from_date(&self) -> String {
        let tz: chrono_tz::Tz = self.timezone.parse().expect("Unknown timezone!");
        self.from_date.with_timezone(&tz).to_rfc2822()
    }

    pub fn local_to_date(&self) -> String {
        let tz: chrono_tz::Tz = self.timezone.parse().expect("Unknown timezone!");
        self.to_date.with_timezone(&tz).to_rfc2822()
    }

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

    pub fn speaker_names(&self) -> String {
        self.speakers.as_ref()
            .map(|speakers| speakers.iter()
                .map(|speaker| format!("{} {}", speaker.first_name, speaker.last_name))
                .collect::<Vec<String>>()
                .join(", "))
            .unwrap_or(String::new())
    }
}

