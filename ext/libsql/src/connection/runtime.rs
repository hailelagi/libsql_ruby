//! runtime.rs

lazy_static::lazy_static! {
    pub static ref TOKIO_RT: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
        .thread_name("libsql-runtime-rb")
        .worker_threads(1)
        .enable_io()
        .build()
        .unwrap();
}
