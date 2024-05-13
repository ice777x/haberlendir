use crate::feed::Feed;
use futures::TryStreamExt;
use mongodb::{bson::doc, Client, Collection};

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
    pub async fn find(&self, regex: &str) -> Vec<Feed> {
        let filter = doc! { "$or"	: [
        doc!{"title": doc!{"$regex": regex, "$options": "i"}},
        doc!{ "content": doc!{"$regex": regex, "$options": "i"}}
        ]};
        let result = self.col.find(filter, None).await;
        match result {
            Ok(cur) => match cur.try_collect().await {
                Ok(dcs) => dcs,
                Err(e) => {
                    println!("{}", e.kind);
                    Vec::new()
                }
            },
            Err(e) => {
                println!("{}", e.kind);
                Vec::new()
            }
        }
    }

    pub async fn insert_one(&self, docx: Feed) {
        let res = self.col.insert_one(docx, None).await;
        match res {
            Ok(ins) => println!("The Doc({}) Successfully inserted", ins.inserted_id),
            Err(_) => println!("Duplicate key error"),
        }
    }

    pub async fn insert_many(&self, docs: Vec<Feed>) {
        let cleaned = self.check_many(docs).await;
        let res = self.col.insert_many(cleaned, None).await;
        match res {
            Ok(imr) => println!("The Docs({}) Successfully inserted", imr.inserted_ids.len()),
            Err(_) => println!("Duplicate key error"),
        }
    }

    pub async fn delete_one(&self, doc: &Feed) -> bool {
        let result = self.col.delete_one(doc! {"title": &doc.title}, None).await;
        result.map(|_| true).unwrap()
    }

    pub async fn check(&self, doc: &Feed) -> bool {
        let result = self
            .col
            .find_one(doc! {"title": doc!{"$eq": doc.title.as_str()}}, None)
            .await;
        match result {
            Ok(docx) => docx.is_some(),
            Err(e) => {
                println!("{}", e.kind);
                true
            }
        }
    }

    pub async fn check_many(&self, docs: Vec<Feed>) -> Vec<Feed> {
        let mut cleaned_docs = Vec::new();
        for doc in docs {
            if !self.check(&doc).await {
                cleaned_docs.push(doc);
            }
        }
        cleaned_docs.dedup_by(|a, b| a.title == b.title && a.link == b.link);
        cleaned_docs
    }
}
