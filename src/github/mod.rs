use std::fs;
use chrono::{DateTime, Utc};

#[derive(Deserialize, Debug)]
pub struct User {
    pub login: String,
    pub id: u32,
}

#[derive(Deserialize, Debug)]
pub struct Repo {
    pub id: u32,
    pub full_name: String,
    pub name: String,
    pub language: Option<String>,
    pub owner: User,
    pub created_at: DateTime<Utc>
}

pub fn get_repos(_user: &'static str) -> Result<Vec<Repo>, failure::Error> {
    let repos = get_repos_from_file();
    repos.map(|repos| {
        let mut repos = repos;
        repos.sort_by_key(|repo| repo.created_at);
        repos
    })
}

#[allow(dead_code)]
fn get_repos_from_github(user: &'static str) -> Result<Vec<Repo>, failure::Error> {

    let request_url = format!("https://api.github.com/users/{owner}/repos", owner = user);
    println!("{}", request_url);
    let mut response = reqwest::get(&request_url)?;

    let repos: Vec<Repo> = response.json().unwrap();
    println!("{:?}", repos);
    Ok(repos)
}

fn get_repos_from_file() -> Result<Vec<Repo>, failure::Error> {
    let contents = fs::read_to_string("repos.json")?;
    serde_json::from_str(&contents).map_err(failure::Error::from)
}
