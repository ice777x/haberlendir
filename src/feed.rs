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
            self.description().unwrap_or_default(),
            self.author
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Feed {
    pub title: String,
    pub link: String,
    pub description: Option<String>,
    pub author: String,
    pub guid: String,
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
    fn image_from_source(&self) -> String {
        let re = Regex::new(r#"src="(?<link>.+)""#).unwrap();
        let desc = self.description();
        match desc {
            Some(desc) => match re.captures(&desc).ok_or(String::new()) {
                Ok(caps) => caps.name("link").unwrap().as_str().to_string(),
                Err(e) => e,
            },
            None => {
                let cont = self.content();
                match cont {
                    Some(text) => match re.captures(&text).ok_or(String::new()) {
                        Ok(caps) => caps
                            .name("link")
                            .ok_or(String::new())
                            .unwrap()
                            .as_str()
                            .to_string(),
                        Err(e) => e,
                    },
                    None => String::new(),
                }
            }
        }
    }

    #[allow(clippy::manual_map)]
    pub fn description(&self) -> Option<String> {
        let re = Regex::new(r#"<(?:"[^"]*"['"]*|'[^']*'['"]*|[^'">])+>"#).unwrap();
        match &self.description {
            Some(desc) => Some(
                re.replace_all(desc, "")
                    .to_string()
                    .replace("&#039;", "'")
                    .replace("&#8216;", "'")
                    .replace("&#8217;", "'")
                    .replace("&#46;", ".")
                    .replace("&amp;", "&")
                    .replace("&quot;", "\"")
                    .replace("\n\n", "\n")
                    .replace("(adsbygoogle = window.adsbygoogle || []).push({});", "")
                    .replace("&#8220;", "\""),
            ),
            None => None,
        }
    }

    #[allow(clippy::manual_map)]
    pub fn content(&self) -> Option<String> {
        let re = Regex::new(r#"<(?:"[^"]*"['"]*|'[^']*'['"]*|[^'">])+>"#).unwrap();
        match &self.content.clone() {
            Some(cont) => Some(
                re.replace_all(cont, "")
                    .to_string()
                    .replace("&#039;", "'")
                    .replace("&#8216;", "'")
                    .replace("&#8217;", "'")
                    .replace("&#46;", ".")
                    .replace("&amp;", "&")
                    .replace("&quot;", "\"")
                    .replace("&#8220;", "\"")
                    .replace("\n\n", "\n")
                    .replace("(adsbygoogle = window.adsbygoogle || []).push({});", ""),
            ),
            None => None,
        }
    }
}
