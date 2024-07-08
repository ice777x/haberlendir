use crate::database::DB;
use axum::extract::{Json, Query, State};
use axum::response::IntoResponse;
use haberlendir_parser::Feed;
use log::trace;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub struct Delete {
    ids: Vec<String>,
    all: bool,
}

pub async fn root() -> &'static str {
    "hello world"
}

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
    Json(items)
}

pub async fn get_news_resourcers(State(db): State<Arc<DB>>) -> Json<Option<Vec<String>>> {
    let authors = db.get_resourcers().await;
    Json::from(authors)
}

pub async fn delete_feeds(
    State(db): State<Arc<DB>>,
    Json(payload): Json<Delete>,
) -> impl IntoResponse {
    let Delete { ids, all } = payload;
    db.delete_many(&ids, all).await;
    (StatusCode::OK, Json(ids))
}
