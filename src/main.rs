pub mod chain_capnp {
    include!(concat!("ipc/capnp/", "chain_capnp.rs"));
}
pub mod common_capnp {
    include!(concat!("ipc/capnp/", "common_capnp.rs"));
}
pub mod echo_capnp {
    include!(concat!("ipc/capnp/", "echo_capnp.rs"));
}
pub mod handler_capnp {
    include!(concat!("ipc/capnp/", "handler_capnp.rs"));
}
pub mod init_capnp {
    include!(concat!("ipc/capnp/", "init_capnp.rs"));
}
pub mod mining_capnp {
    include!(concat!("ipc/capnp/", "mining_capnp.rs"));
}
pub mod proxy_capnp {
    include!(concat!("ipc/capnp/", "proxy_capnp.rs"));
}

fn main() {
    println!("Hello world!");
}