use futures::TryStreamExt;
use haberlendir_parser::Feed;
use mongodb::{bson::doc, Client, Collection};

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

    pub async fn get_all(&self) -> Vec<Feed> {
        let filter = doc! { "$or"	: [
        doc!{"title": doc!{"$regex": "", "$options": "i"}},
        doc!{ "content": doc!{"$regex": "", "$options": "i"}}
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
            Ok(ins) => println!("Doc({}) Successfully inserted", ins.inserted_id),
            Err(_) => println!("Duplicate key error"),
        }
    }

    pub async fn insert_many(&self, docs: Vec<Feed>) {
        let cleaned = self.check_many(docs).await;
        let res = self.col.insert_many(cleaned, None).await;
        match res {
            Ok(imr) => println!("Docs({}) Successfully inserted", imr.inserted_ids.len()),
            Err(e) => {
                println!("Duplicate key error");
                println!("{}", e.kind);
            }
        }
    }

    pub async fn delete_one(&self, doc: Feed) -> bool {
        let result = self.col.delete_one(doc! {"title": &doc.title}, None).await;
        result.map(|_| true).unwrap()
    }

    pub async fn delete_many(&self) {
        let filter = doc! { "$or"	: [
        doc!{"title": doc!{"$regex": "", "$options": "i"}},
        doc!{ "content": doc!{"$regex": "", "$options": "i"}}
        ]};
        match self.col.delete_many(filter, None).await {
            Ok(_) => (),
            Err(e) => println!("{:?}", e.kind),
        };
    }

    async fn check(&self, doc: &Feed) -> bool {
        let result = self.col.find_one(doc! {"id": doc.id.as_str()}, None).await;
        match result {
            Ok(docx) => docx.is_some(),
            Err(e) => {
                println!("Check Func Error: {}", e.kind);
                true
            }
        }
    }

    async fn check_many(&self, docs: Vec<Feed>) -> Vec<Feed> {
        let mut clean_docs = Vec::new();
        for doc in docs {
            if !self.check(&doc).await && !clean_docs.contains(&doc) {
                clean_docs.push(doc);
            }
        }
        clean_docs
    }
}

#[derive(Default)]
pub struct Scheduler {
    pub state: bool,
    pub t_per_minute: u64,
}

impl Scheduler {
    pub fn new(t_per_minute: u64) -> Self {
        Self {
            t_per_minute,
            state: true,
        }
    }

    pub fn update(&mut self, update_value: bool) {
        self.state = update_value;
    }
}
