use yew::prelude::*;

pub(crate) mod workerd_capnp {
    include!(concat!(env!("OUT_DIR"), "/workerd_capnp.rs"));
}

mod app_home;
mod nav_header;
mod services_table;
mod sockets_table;
mod pop_modal;
mod pop_modal_overlay;
mod workers_table;

use app_home::AppHome;
use nav_header::NavHeader;
use services_table::ServicesTable;
use sockets_table::SocketsTable;
use pop_modal::PopModal;
use pop_modal_overlay::PopModalOverlay;
use workers_table::WorkersTable;

fn main() {
    yew::Renderer::<AppHome>::new().render();
}
