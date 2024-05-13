use crate::feed::Feed;
use feed_rs::parser::parse;
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

    pub async fn get_rss(&self, url: &str) -> Result<Vec<Feed>, Box<dyn std::error::Error>> {
        let res = reqwest::get(url).await?.bytes().await?;
        let rss = parse(&res[..])?;
        let mut items = Vec::new();
        let source = &rss.title.ok_or(String::new()).unwrap().content;
        for entry in rss.entries {
            let feed = Feed {
                guid: entry.id,
                title: entry.title.ok_or(String::new()).unwrap().content,
                author: source.to_string(),
                link: match entry.links.first() {
                    Some(link) => link.href.clone(),
                    None => String::new(),
                },
                description: match entry.summary {
                    Some(t) => Some(t.content),
                    None => None,
                },
                content: match entry.content {
                    Some(m) => Some(m.body.unwrap_or(String::new())),
                    None => None,
                },
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
