pub(crate) mod workerd_capnp {
    include!(concat!(env!("OUT_DIR"), "/workerd_capnp.rs"));
}

mod server;
mod store;
mod workerd;

use store::Store;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = Store::new().await;
    store.create_worker().await?;
    workerd::write_workerd_config().unwrap();
    server::run().await
}
