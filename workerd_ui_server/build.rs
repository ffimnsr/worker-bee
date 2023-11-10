fn main() {
    capnpc::CompilerCommand::new()
        .file("workerd.capnp")
        .run()
        .expect("=> compiling schema");
}
