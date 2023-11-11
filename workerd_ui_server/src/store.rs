use serde::{Deserialize, Serialize};
use surrealdb::engine::local::{RocksDb, Db};
use surrealdb::opt::Config;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Socket {
  pub name: String,
  pub address: String,
  pub service_id: Thing,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Service {
  pub name: String,
  pub worker: String,
  pub worker_id: Thing,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Worker {
  pub name: String,
  pub service_worker_script: String,
  pub compatibility_date: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Record {
  #[allow(dead_code)]
  id: Thing
}

#[derive(Deserialize, Debug)]
pub(crate) struct WorkerRecord {
  id: Thing,
  name: String,
  service_worker_script: String,
  compatibility_date: String,
}


#[derive(Debug)]
pub(crate) struct Store {
  db: Surreal<Db>,
}

#[derive(Debug)]
pub(crate) struct ConfigData {
    workers: Vec<WorkerRecord>,
    services: Vec<Service>,
    sockets: Vec<Socket>,
}

impl Store {
  pub async fn new() -> Self {
    let config = Config::default();
    let db = Surreal::new::<RocksDb>(("/tmp/temp.db", config))
      .await
      .expect("failed to initialize db");

    db.use_ns("workerbee").use_db("workerd").await.expect("failed to use db");

    Self { db }
  }



  pub async fn get_all(&self) -> Vec<WorkerRecord> {
    let db = self.db.clone();
    let workers: Vec<WorkerRecord> = db
      .select("worker")
      .await
      .expect("failed to get workers");

    workers
    // let services: Vec<Record> = db
    //   .select("service")
    //   .await
    //   .expect("failed to get services");

    // let sockets: Vec<Record> = db
    //   .select("socket")
    //   .await
    //   .expect("failed to get sockets");

  }

  pub async fn create_worker(&self, data: &Worker) -> surrealdb::Result<Vec<Record>> {
    let db = self.db.clone();

    let created: Vec<Record> = db
      .create("worker")
      .content(data).await?;

    Ok(created)
  }

  pub async fn create_service(&self, data: &Service) -> surrealdb::Result<Vec<Record>> {
    let db = self.db.clone();

    let created: Vec<Record> = db
      .create("service")
      .content(data).await?;

    Ok(created)
  }

  pub async fn create_socket(&self, data: &Socket) -> surrealdb::Result<Vec<Record>> {
    let db = self.db.clone();

    let created: Vec<Record> = db
      .create("socket")
      .content(data).await?;

    Ok(created)
  }
}
