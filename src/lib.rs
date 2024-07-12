use std::sync::Arc;

use database::DB;
use haberlendir_parser::Parser;

pub mod database;
pub mod logger;
pub mod middleware;
pub mod router;

pub async fn create(db: Arc<DB>) {
    let parser = Parser::build("source.json").await;
    let items = parser.get_all_rss().await;
    db.insert_many(items).await;
}
