use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Json, Query, State},
    http::StatusCode,
    routing::get,
    Router,
};
use conc::database::DB;
use dotenvy::dotenv;
use haberlendir_parser::{Feed, Parser};
use mongodb::Client;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    dotenv().ok();
    let client = Client::with_uri_str(
        std::env::var("MONGODB_URI").expect("Please set MONGODB_URI in .env file"),
    )
    .await?;
    let db = Arc::new(DB::new(&client, "haberlendir_news", "news"));
    let app = Router::new()
        .route("/", get(root))
        .route("/find", get(get_news_with_query))
        .with_state(db);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn root() -> &'static str {
    "hello world"
}

async fn get_news_with_query(
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
