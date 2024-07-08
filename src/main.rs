use axum::{
    routing::{delete, get, get_service},
    Router,
};
use clokwerk::{AsyncScheduler, TimeUnits};
use conc::{create, database::DB, middleware::auth, router::*};
use dotenvy::dotenv;
use log::info;
use mongodb::Client;
use std::{sync::Arc, time::Duration};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    pretty_env_logger::init();
    dotenv().ok();
    let client = Client::with_uri_str(
        std::env::var("MONGODB_URI").expect("Please set MONGODB_URI in .env file"),
    )
    .await?;
    let db = Arc::new(DB::new(&client, "haberlendir_news", "news"));
    let temp_db = Arc::clone(&db);
    let mut sched = AsyncScheduler::new();
    sched.every(45.seconds()).run(move || {
        let temp_db = Arc::clone(&temp_db);
        async move {
            create(temp_db).await;
        }
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let api = Router::new()
        .route("/", get(root))
        .route("/find", get(get_news_with_query))
        .route("/delete", delete(delete_feeds))
        .route("/resourcers", get(get_news_resourcers))
        .route_layer(axum::middleware::from_fn(auth));

    let app = Router::new()
        .nest("/api", api)
        .nest_service("/", get_service(ServeDir::new("../frontend/dist")))
        .layer(cors)
        .with_state(db);

    let listener = tokio::net::TcpListener::bind(&"0.0.0.0:3000")
        .await
        .unwrap();

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
