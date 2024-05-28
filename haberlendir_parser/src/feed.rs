use html_escape::decode_html_entities;
use regex::Regex;
use serde::{Deserialize, Serialize};

pub trait UI {
    fn write(&self);
}

impl UI for Feed {
    fn write(&self) {
        println!(
            "{}\n\n{}\n@{}\n",
            self.title.to_uppercase(),
            self.description().as_ref().unwrap(),
            self.author
        )
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Feed {
    pub id: String,
    pub title: String,
    pub link: String,
    pub description: Option<String>,
    pub author: String,
    pub images: Option<String>,
    pub published: Option<String>,
    pub content: Option<String>,
}

impl Feed {
    // fn to_date(date: &str) -> NaiveDateTime {
    //     let new_date = NaiveDateTime::parse_and_remainder(date, "%Y-%m-%d %H:%M:%S").ok();
    //     // let tz = FixedOffset::east_opt(3 * 3600).unwrap();
    //     match new_date {
    //         Some((date, _)) => date,
    //         None =>
    //             NaiveDateTime::parse_and_remainder(date, "%a, %d %b %Y %H:%M:%S %z")
    //                 .unwrap()
    //                 .0
    //         }
    //     }
    // }

    #[allow(dead_code)]
    fn image_from_source(&self) -> Option<String> {
        let re = Regex::new(r#"src="(?<link>.+)""#).unwrap();
        let desc = self.description();
        match desc {
            Some(desc) => re
                .captures(&desc)
                .map(|caps| caps.name("link").unwrap().as_str().to_string()),
            None => {
                let cont = self.content();
                match cont {
                    Some(text) => re
                        .captures(&text)
                        .map(|caps| caps.name("link").unwrap().as_str().to_string()),
                    None => None,
                }
            }
        }
    }

    #[allow(clippy::manual_map)]
    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    #[allow(clippy::manual_map)]
    pub fn content(&self) -> Option<String> {
        let re = Regex::new(r#"<(?:"[^"]*"['"]*|'[^']*'['"]*|[^'">])+>"#).unwrap();
        match &self.content {
            Some(cont) => {
                let str_without_tags = re.replace_all(cont, "");
                Some(decode_html_entities(&str_without_tags).to_string())
            }
            None => None,
        }
    }
}
