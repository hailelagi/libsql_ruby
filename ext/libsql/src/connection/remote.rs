//! Creates a connection to a remote database.
/// This is a blocking IO call that suspends the entire ruby VM.
/// Eventually it would be nice to do something like:
/// https://github.com/bytecodealliance/wasmtime-rb/blob/main/ext/src/helpers/nogvl.rs
/// Alternatively maybe spawn a thread throw upwards?
///  
use crate::connection::runtime;

pub fn open_remote_connection(url: String, auth_token: String) -> libsql::Connection {
    let conn = runtime::TOKIO_RT.block_on(async {
        let db = libsql::Builder::new_remote(url, auth_token)
            .build()
            .await
            .unwrap();
        let conn = db.connect().unwrap();
        conn
    });

    conn
}
