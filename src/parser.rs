use crate::feed::Feed;
use feed_rs::parser::parse;
use html_escape::decode_html_entities;
use regex::Regex;
use tokio::fs;

pub struct Parser {
    pub urls: Vec<String>,
}

impl Parser {
    pub async fn build(path: &str) -> Parser {
        let urls = Parser::news_from_file(path).await;
        Parser { urls }
    }

    async fn news_from_file(path: &str) -> Vec<String> {
        let read_dir = fs::read_to_string(path).await;
        match read_dir {
            Ok(text) => text.lines().map(|f| f.to_string()).collect(),
            Err(e) => {
                println!("{}", e.kind());
                Vec::new()
            }
        }
    }

    /// Returns the get all rss of this [`Parser`].
    pub async fn get_all_rss(&self) -> Vec<Feed> {
        let mut items = Vec::new();
        let mut rss = Vec::new();
        for i in self.urls.iter() {
            items.push(self.get_rss(i));
        }
        let pool = futures::future::join_all(items).await;
        for i in pool.into_iter().flatten() {
            rss.extend(i);
        }
        rss
    }

    #[allow(clippy::manual_map)]
    fn description(&self, content: String) -> String {
        let re = Regex::new(r"<.*?>").unwrap();
        let text_without_tags = re.replace_all(&content, "");
        decode_html_entities(&text_without_tags).to_string()
    }

    #[allow(clippy::manual_map)]
    pub fn content(&self, content: String) -> String {
        let re = Regex::new(r#"<(?:"[^"]*"['"]*|'[^']*'['"]*|[^'">])+>"#).unwrap();
        let str_without_tags = re
            .replace_all(&content, "")
            .replace("(adsbygoogle = window.adsbygoogle || []).push({});", "")
            .replace("\n\n", "\n");
        decode_html_entities(&str_without_tags).to_string()
    }

    pub async fn get_rss(&self, url: &str) -> Result<Vec<Feed>, Box<dyn std::error::Error>> {
        let res = reqwest::get(url).await?.bytes().await?;
        let rss = parse(&res[..])?;
        let mut items = Vec::new();
        let source = &rss.title.ok_or(String::new()).unwrap().content;
        for entry in rss.entries {
            if entry.title.is_none() {
                continue;
            }
            let feed = Feed {
                id: entry.id,
                title: entry.title.ok_or(String::new()).unwrap().content,
                author: source.to_string(),
                link: match entry.links.first() {
                    Some(link) => link.href.clone(),
                    None => String::new(),
                },
                description: if let Some(t) = entry.summary {
                    Some(self.description(t.content))
                } else {
                    None
                },
                content: entry
                    .content
                    .map(|content| self.content(content.body.unwrap_or_default())),
                published: entry.published.map(|publ| publ.to_string()),
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
            };
            items.push(feed);
        }
        Ok(items)
    }
}
