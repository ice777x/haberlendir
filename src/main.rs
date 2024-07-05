use axum::{routing::get, Router};
use clokwerk::{AsyncScheduler, TimeUnits};
use conc::{database::DB, router::*};
use dotenvy::dotenv;
use log::info;
use mongodb::{options::ClientOptions, Client};
use std::{collections::HashMap, sync::Arc};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    dotenv().ok();
    let client = Client::with_uri_str(
        std::env::var("MONGODB_URI").expect("Please set MONGODB_URI in .env file"),
    )
    .await?;
    let db = DB::new(&client, "haberlendir_news", "news");
    let t_db = Arc::new(db);
    let mut sched = AsyncScheduler::new();

    sched.every(10.minutes()).run(move || async {
        let s = Arc::clone(&t_db);
        create(s).await;
    });

    let db4 = Arc::new(db);
    let app = Router::new()
        .route("/", get(root))
        .route("/find", get(get_news_with_query))
        // .route("/create", get(create))
        .with_state(db4);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!("Starting Server on :3000");
    axum::serve(listener, app).await.unwrap();

    // tokio::spawn(async move {
    //     let mut scheduler = AsyncScheduler::new();

    //     scheduler.every(10.minutes()).run(|| async {
    //         reqwest::get("localhost:3000/create").await.unwrap();
    //     });
    //     scheduler.run_pending().await;
    //     println!("Running");
    // });
    Ok(())
}
