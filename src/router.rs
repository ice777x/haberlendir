use crate::database::DB;
use axum::extract::{Json, Query, State};
use haberlendir_parser::Feed;
use log::trace;
use mongodb::bson::doc;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub struct Delete {
    ids: Option<Vec<String>>,
    all: bool,
}

#[derive(Serialize, Deserialize)]
pub struct HPath {
    endpoint: &'static str,
    usage: &'static str,
    parameters: Option<HashMap<&'static str, &'static str>>,
}
pub async fn root() -> Json<Vec<HPath>> {
    let find_parameters: HashMap<&str, &str> = HashMap::from_iter([
        ("q", "query"),
        ("limit", "limit default value is 10"),
        ("skip", "skip default value is 0"),
        ("author", "for searching authors false or true"),
    ]);
    let path: Vec<HPath> = vec![
        HPath {
            endpoint: "/api",
            usage: "/api",
            parameters: None,
        },
        HPath {
            endpoint: "/find",
            usage: "/api/find?q=bugün&limit=10&skip=20&author=false",
            parameters: Some(find_parameters),
        },
        HPath {
            endpoint: "/resourcers",
            usage: "/api/resourcers",
            parameters: None,
        },
    ];
    Json(path)
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
    let author = query.get("author").map(|aut| aut.parse::<bool>().unwrap());
    let items = db.find(q, Some(limit), author, Some(skip)).await;
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
) -> (StatusCode, Json<Option<Vec<String>>>) {
    let Delete { ids, all } = payload;
    if ids.is_some() {
        let ids = ids.unwrap();
        db.delete_many(Some(&ids), all).await;
        (StatusCode::OK, Json(Some(ids)))
    } else {
        db.delete_many(None, all).await;
        (StatusCode::OK, Json(None))
    }
}
