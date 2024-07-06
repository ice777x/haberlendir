use crate::database::DB;
use axum::extract::{Json, Query, State};
use haberlendir_parser::{Feed, Parser};
use log::trace;
use std::collections::HashMap;
use std::sync::Arc;

pub async fn get_news_with_query(
    Query(query): Query<HashMap<String, String>>,
    State(db): State<Arc<DB>>,
) -> Json<Vec<Feed>> {
    let mut limit: i64 = 10;
    let mut skip: u64 = 0;
    if query.contains_key("limit") {
        limit = query.get("limit").unwrap().parse::<i64>().unwrap_or(10);
    }

    if query.contains_key("skip") {
        skip = query.get("skip").unwrap().parse::<u64>().unwrap_or(10);
    }

    let q = query.get("q");
    let items = db.find(q, Some(limit), Some(skip)).await;
    trace!(
        "Query: {}, Size:{}",
        q.unwrap_or(&"".to_owned()),
        items.len()
    );
    Json::from(items)
}

pub async fn root() -> &'static str {
    "hello world"
}

pub async fn delete_feeds(Json(payload): Json<serde_json::Value>) {
    println!("{:?}", payload);
}
