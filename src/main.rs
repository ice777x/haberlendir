use axum::{
    routing::{delete, get},
    Router,
};
use clokwerk::{AsyncScheduler, TimeUnits};
use conc::{create, database::DB, router::*};
use dotenvy::dotenv;
use log::info;
use mongodb::Client;
use std::{sync::Arc, time::Duration};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    pretty_env_logger::init();
    dotenv().ok();
    let client = Client::with_uri_str(
        std::env::var("MONGODB_URI").expect("Please set MONGODB_URI in .env file"),
    )
    .await?;
    let db = Arc::new(DB::new(&client, "haberlendir_news", "news"));
    let mut sched = AsyncScheduler::new();
    let temp_db = Arc::clone(&db);
    sched.every(45.seconds()).run(move || {
        let temp_db = Arc::clone(&temp_db);

        async move {
            create(temp_db).await;
        }
    });

    let api = Router::new()
        .route("/", get(root))
        .route("/find", get(get_news_with_query))
        .route("/delete", delete(delete_feeds));
    let app = Router::new().nest("/api", api).with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!("Starting Server on :3000");
    tokio::spawn(async move {
        info!("Scheduler Spawned");
        loop {
            sched.run_pending().await;
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
