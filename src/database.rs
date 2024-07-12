use futures::{StreamExt, TryStreamExt};
use haberlendir_parser::Feed;
use log::{error, info};
use mongodb::{bson::doc, options::FindOptions, Client, Collection};
use std::collections::HashSet;

#[derive(Clone)]
pub struct DB {
    pub col: Collection<Feed>,
}

impl DB {
    pub fn new(client: &Client, db_name: &str, col_name: &str) -> Self {
        Self {
            col: client.database(db_name).collection(col_name),
        }
    }

    /// Find item in database with RegEx
    /// # Examples
    /// ```
    ///
    /// let cl = Client::with_uri_str("mongodb_url").await?;
    /// let db = DB::new(&cl, "db_name", "col_name");
    /// let items = db.find("\\w+inci").await;
    ///
    /// ```
    pub async fn find(
        &self,
        regex: Option<&String>,
        limit: Option<i64>,
        author: Option<bool>,
        skip: Option<u64>,
    ) -> Vec<Feed> {
        let filter = match regex {
            Some(re) => {
                let mut queries = Vec::new();
                if author.is_none() {
                    queries.push(doc! {"title": doc!{"$regex": re, "$options": "i"}});
                    queries.push(doc! { "content": doc!{"$regex": re, "$options": "i"}});
                    queries.push(doc! { "description": doc!{"$regex": re, "$options": "i"}});
                } else {
                    if author.unwrap() == true {
                        queries.push(doc! {"author": doc!{"$regex": re, "$options": "i"}});
                    } else {
                        queries.push(doc! {"title": doc!{"$regex": re, "$options": "i"}});
                        queries.push(doc! { "content": doc!{"$regex": re, "$options": "i"}});
                        queries.push(doc! { "description": doc!{"$regex": re, "$options": "i"}});
                    }
                }
                doc! { "$or"	:queries}
            }
            None => {
                doc! {}
            }
        };
        let options = FindOptions::builder()
            .sort(doc! {"created_at": -1})
            .limit(limit)
            .skip(skip)
            .build();
        let result = self.col.find(filter).with_options(options).await;
        match result {
            Ok(cur) => match cur.try_collect().await {
                Ok(dcs) => dcs,
                Err(e) => {
                    error!("Error: {}", e.kind);
                    Vec::new()
                }
            },
            Err(e) => {
                error!("DB Collect Error: {}", e.kind);
                Vec::new()
            }
        }
    }

    pub async fn insert_one(&self, docx: Feed) {
        let res = self.col.insert_one(docx).await;
        match res {
            Ok(ins) => info!("{} Document Successfully inserted", ins.inserted_id),
            Err(_) => error!("Duplicate key error"),
        }
    }

    pub async fn insert_many(&self, docs: Vec<Feed>) {
        let cleaned = self.check_many(docs).await;
        if cleaned.is_empty() {
            return;
        }
        let res = self.col.insert_many(cleaned).await;
        match res {
            Ok(imr) => info!("Docs({}) Successfully inserted", imr.inserted_ids.len()),
            Err(e) => {
                error!("Insert Error: {}", e.kind);
            }
        }
    }

    pub async fn delete_one(&self, doc: Feed) -> bool {
        let result = self.col.delete_one(doc! {"id": &doc.id}).await;
        result.map(|_| true).unwrap()
    }

    pub async fn delete_many(&self, docs: Option<&[String]>, all: bool) {
        if docs.is_none() && !all {
            return ();
        }
        // let ids: Vec<&str> = docs.iter().map(|doc| doc.id.as_str()).collect();
        let filter = if all {
            doc! {}
        } else {
            doc! {"id": {"$in": docs}}
        };

        match self.col.delete_many(filter).await {
            Ok(_) => (),
            Err(e) => info!("Delete Error: {:?}", e.kind),
        };
    }

    pub async fn get_resourcers(&self) -> Option<Vec<String>> {
        let result = self.col.distinct("author", doc! {}).await;
        match result {
            Ok(res) => Some(
                res.iter()
                    .map(|res| res.to_string().trim_matches('"').to_string())
                    .collect(),
            ),
            Err(e) => {
                error!("Error: {}", e.kind);
                None
            }
        }
    }

    pub async fn check(&self, doc: &Feed) -> bool {
        let result = self.col.find_one(doc! {"id": doc.id.as_str()}).await;
        match result {
            Ok(docx) => docx.is_some(),
            Err(e) => {
                println!("Check Func Error: {}", e.kind);
                true
            }
        }
    }

    async fn check_many(&self, docs: Vec<Feed>) -> Vec<Feed> {
        let ids: Vec<&str> = docs.iter().map(|doc| doc.id.as_str()).collect();
        let filter = doc! {"id": {"$in": &ids}};
        let existing_docs = self
            .col
            .find(filter)
            .await
            .unwrap()
            .collect::<Vec<_>>()
            .await;
        let existing_ids: HashSet<String> = existing_docs
            .iter()
            .filter_map(|res| match res {
                Ok(dc) => Some(dc.id.clone()),
                Err(_) => None,
            })
            .collect();
        docs.into_iter()
            .filter(|doc| !existing_ids.contains(&doc.id))
            .collect()
    }
}
