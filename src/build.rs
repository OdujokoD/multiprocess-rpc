fn main() -> Result<(), Box<dyn std::error::Error>> {
    capnpc::CompilerCommand::new()
        .src_prefix("src/ipc/capnp")
        .file("src/ipc/capnp/chain.capnp")
        .file("src/ipc/capnp/common.capnp")
        .file("src/ipc/capnp/echo.capnp")
        .file("src/ipc/capnp/handler.capnp")
        .file("src/ipc/capnp/init.capnp")
        .file("src/ipc/capnp/mining.capnp")
        .file("src/ipc/capnp/proxy.capnp")
        .output_path("src/ipc/capnp")
        .run()?;
    
    Ok(())
}