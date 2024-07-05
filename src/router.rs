use crate::database::DB;
use axum::extract::{Json, Query, State};
use haberlendir_parser::Feed;
use std::collections::HashMap;
use std::sync::Arc;

pub async fn get_news_with_query(
    Query(query): Query<HashMap<String, String>>,
    State(db): State<Arc<DB>>,
) -> Json<Vec<Feed>> {
    let mut limit: usize = 10;
    let mut skip: usize = 0;
    if query.contains_key("limit") {
        limit = query.get("limit").unwrap().parse::<usize>().unwrap_or(10);
    }

    if query.contains_key("skip") {
        skip = query.get("skip").unwrap().parse::<usize>().unwrap_or(10);
    }

    if !query.contains_key("q") {
        let mut items = db.get_all().await;
        let end = if (limit + skip) > items.len() {
            items.len()
        } else {
            limit + skip
        };
        if items.len() < skip {
            skip = items.len();
        }
        Json::from(items.drain(skip..end).collect::<Vec<Feed>>())
    } else {
        let mut items = db.find(query.get("q").unwrap().as_str()).await;
        let end = if (limit + skip) > items.len() {
            items.len()
        } else {
            limit + skip
        };
        if items.len() < skip {
            skip = items.len();
        }
        Json::from(items.drain(skip..end).collect::<Vec<Feed>>())
    }
}

pub async fn root() -> &'static str {
    "hello world"
}

pub async fn create(db: Arc<DB>) -> Json<HashMap<String, bool>> {
    let items = db.get_all().await;
    db.insert_many(items).await;
    Json::from(HashMap::from([(String::from("state"), false)]))
}
