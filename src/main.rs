use conc::{database::*, feed::UI};
use dotenvy::dotenv;
use mongodb::Client;
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    dotenv().ok();
    let client_uri = env::var("MONGODB_URI").expect("You must be set database url");
    let cl = Client::with_uri_str(client_uri.as_str()).await?;
    let db = Arc::new(DB::new(&cl, "haberlendir_news", "news"));
    let items = db.find("Türk\\w+").await;
    setup(Arc::clone(&db)).await;
    Ok(())
}

async fn setup(db: Arc<DB>) {
    let items = db.find("Türk\\w+").await;
    println!("{:#?}", items);
}
