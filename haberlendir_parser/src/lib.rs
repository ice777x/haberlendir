pub mod feed;

pub use crate::feed::Feed;

use chrono::Utc;
use feed::Source;
use feed_rs::parser::parse;
use html_escape::decode_html_entities;
use lazy_static::lazy_static;
use log::warn;
use regex::Regex;
use tokio::fs;

lazy_static! {
    static ref RE_TAG: Regex = Regex::new(r"<[^>]+>").unwrap();
    static ref RE_WHITESPACE: Regex = Regex::new(r"\s+").unwrap();
}

fn clean_html(input: &str) -> String {
    let no_tags = RE_TAG.replace_all(input, "");

    // Fazladan boşlukları sil
    let cleaned = RE_WHITESPACE.replace_all(&no_tags, " ").to_string();

    cleaned.trim().to_string()
}

async fn source_from_json(path: &str) -> Vec<Source> {
    let read_file = fs::read_to_string(path)
        .await
        .expect(format!("Can't read data from {path}").as_str());
    let data =
        serde_json::from_str::<Vec<Source>>(&read_file).expect("JSON was not well-formatted");
    data
}

pub struct Parser {
    pub source: Vec<Source>,
}

impl Parser {
    pub async fn build(path: &str) -> Parser {
        let source = source_from_json(path).await;
        Parser { source }
    }

    /// Returns the get all rss of this [`Parser`].
    pub async fn get_all_rss(&self) -> Vec<Feed> {
        let mut rss = Vec::new();
        for i in self.source.iter() {
            if let Ok(news) = self.get_rss(&i.rss, i).await {
                rss.extend(news);
            }
        }
        rss
    }

    pub async fn get_rss(
        &self,
        url: &str,
        src: &Source,
    ) -> Result<Vec<Feed>, Box<dyn std::error::Error>> {
        let res = reqwest::get(url).await?.bytes().await?;
        let rss = parse(&res[..])?;
        let mut items = Vec::new();

        let source = match &rss.title.ok_or("Resource Title Not Found") {
            Ok(src) => src.content.to_owned(),
            Err(e) => {
                warn!("{:?} {}", url, e);
                println!("D");
                "".to_owned()
            }
        };

        if source == String::new() {
            return Ok(Vec::new());
        }

        for entry in rss.entries {
            if entry.title.is_none() {
                continue;
            }

            if entry.links.first().is_none() {
                continue;
            }
            let categories = entry
                .categories
                .into_iter()
                .map(|cat| cat.term)
                .collect::<Vec<String>>();
            let author = entry
                .authors
                .into_iter()
                .map(|aut| aut.name)
                .collect::<Vec<String>>();
            let feed = Feed {
                id: entry.id,
                title: entry.title.ok_or(String::new()).unwrap().content,
                author,
                source: src.clone(),
                link: match entry.links.first() {
                    Some(link) => link.href.clone(),
                    None => String::new(),
                },
                description: if let Some(t) = entry.summary {
                    Some(self.description(&t.content))
                } else {
                    None
                },
                categories,
                content: entry
                    .content
                    .map(|content| self.content(&content.body.unwrap_or_default())),
                published: entry.published,
                images: match entry.media.first() {
                    Some(med) => {
                        let img = match med.content.first() {
                            Some(mc) => mc.url.as_ref().map(|url| url.to_string()),
                            None => None,
                        };
                        img
                    }
                    None => None,
                },
                created_at: Utc::now(),
            };
            items.push(feed);
        }
        Ok(items)
    }

    #[allow(clippy::manual_map)]
    fn description(&self, content: &str) -> String {
        let cleaned = clean_html(content);
        decode_html_entities(&cleaned).to_string()
    }

    #[allow(clippy::manual_map)]
    pub fn content(&self, content: &str) -> String {
        let cleaned = clean_html(content)
            .replace("(adsbygoogle = window.adsbygoogle || []).push({});", "")
            .replace("\n\n", "\n")
            .trim()
            .to_owned();
        decode_html_entities(&cleaned).to_string()
    }
}
