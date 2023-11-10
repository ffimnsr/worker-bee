use std::sync::Arc;

use serde::{Deserialize, Serialize};
use surrealdb::engine::local::{RocksDb, Db};
use surrealdb::opt::Config;
use surrealdb::Surreal;
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize, Debug)]
struct Socket {
  name: String,
  address: String,
  service_id: Thing,
}

#[derive(Serialize, Deserialize, Debug)]
struct Service {
  name: String,
  worker: String,
  worker_id: Thing,
}

#[derive(Serialize, Deserialize, Debug)]
struct Worker {
  service_worker_script: String,
  compatibility_date: String,
}

#[derive(Deserialize, Debug)]
struct Record {
  #[allow(dead_code)]
  id: Thing
}

pub(crate) struct Store {
  db: Arc<Surreal<Db>>,
}

impl Store {
  pub async fn new() -> Self {
    let config = Config::default().strict();
    let db = Surreal::new::<RocksDb>(("temp.db", config))
      .await
      .expect("failed to initialize db");

    Self {
      db: Arc::new(db),
    }
  }

  pub async fn create_worker(&self) -> surrealdb::Result<()> {
    let db = self.db.clone();
    let created: Vec<Record> = db
      .create("worker")
      .content(Worker {
        service_worker_script: "service_worker_script".to_string(),
        compatibility_date: "compatibility_date".to_string(),
      }).await?;

    dbg!(created);
    Ok(())
  }
}
