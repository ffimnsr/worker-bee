use std::fs::{File, self};

use crate::workerd_capnp::{config, service_designator, worker};
use capnp::{message::TypedBuilder, serialize};

pub fn write_workerd_config() -> capnp::Result<()> {
    let mut message: TypedBuilder<config::Owned> = TypedBuilder::<config::Owned>::new_default();

    {
        let mut worker_message = TypedBuilder::<worker::Owned>::new_default();
        let mut config = message.init_root();
        let worker = create_workerd_worker(&mut worker_message)?;
        let mut services = config.reborrow().init_services(1);
        let mut service = services.reborrow().get(0);
        service.set_name("main".into());
        service.set_worker(worker)?;

        let mut service_designator_message = TypedBuilder::<service_designator::Owned>::new_default();
        let service_designator =
            create_workerd_service_designator(&mut service_designator_message)?;
        let mut sockets = config.init_sockets(1);
        let mut socket = sockets.reborrow().get(0);
        socket.set_name("http".into());
        socket.set_address("*:8080".into());
        socket.set_service(service_designator)?;
    }

    let binding = std::env::current_dir().expect("cwd should not be empty");
    let cwd = binding.to_str().expect("cwd should not be empty");
    let path = format!("{cwd}/workerd-config.bin");
    let file = File::create(path)?;

    serialize::write_message(file, message.borrow_inner())?;
    Ok(())
}

pub fn create_workerd_worker<'a>(
    message: &'a mut TypedBuilder<worker::Owned>,
) -> capnp::Result<worker::Reader<'a>> {
    let binding = std::env::current_dir().expect("cwd should not be empty");
    let cwd = binding.to_str().expect("cwd should not be empty");
    let path = format!("{cwd}/workers/helloworld/worker.js");
    if fs::metadata(path.clone()).is_err() {
        panic!("worker.js not found");
    }

    let script = fs::read_to_string(path).expect("something went wrong reading the file");

    let mut worker = message.init_root();
    worker.set_compatibility_date("2023-10-30".into());
    worker.set_service_worker_script(script.as_str().into());
    message.get_root_as_reader()
}

pub fn create_workerd_service_designator<'a>(
    message: &'a mut TypedBuilder<service_designator::Owned>,
) -> capnp::Result<service_designator::Reader<'a>> {
    let mut service_designator = message.init_root();
    service_designator.set_name("main".into());
    message.get_root_as_reader()
}
