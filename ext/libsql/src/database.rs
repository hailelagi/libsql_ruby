//! Database.rs

use std::cell::RefCell;
use std::sync::{Arc, Mutex};

use libsql;
use libsql::{Builder, Connection};
use magnus::{
    class, define_class, exception, function, method, module,
    prelude::*,
    scan_args::{get_kwargs, scan_args},
    typed_data, Error, Value,
};

use crate::connection::runtime::TOKIO_RT;

pub enum Mode {
    Remote,
    Local,
    RemoteReplica,
}

#[derive(Default)]
#[magnus::wrap(class = "LibSQL::Database")]
pub(crate) struct Database {
    conn: RefCell<Option<Arc<Mutex<libsql::Connection>>>>,
    db: RefCell<Option<Arc<Mutex<libsql::Database>>>>
}

impl Database {
    pub fn new(db: libsql::Database, conn: libsql::Connection) -> Self {
        Database {
            db: RefCell::new(Some(Arc::new(Mutex::new(db)))),
            conn: RefCell::new(Some(Arc::new(Mutex::new(conn)))),
        }
    }

    pub fn initialize(_rb_self: typed_data::Obj<Self>, args: &[Value]) -> Result<Database, Error> {
        let args = scan_args::<(), (), (), (), _, ()>(args)?;
        let args = get_kwargs(args.keywords, &[], &["url"])?;

        let _: () = args.required;
        let (url,): (Option<String>,) = args.optional;
        let _: () = args.splat;
        let url = url.unwrap_or_else(|| String::from("test.db"));

        let db = TOKIO_RT.block_on(async { libsql::Builder::new_local(url).build().await}).unwrap();
        let conn = TOKIO_RT.block_on(async { db.connect() }).unwrap();

        Ok(Database::new(db, conn))
    }
}

#[cfg(test)]
mod tests {
    use rb_sys::{rb_int2big, rb_num2fix, FIXNUM_P};
    use rb_sys_test_helpers::ruby_test;

    #[ruby_test]
    fn test_something() {
        // Your test code here will have a valid Ruby VM (hint: this works with
        // the `magnus` crate, too!)
        //
        // ...

        let int = unsafe { rb_num2fix(1) };
        let big = unsafe { rb_int2big(9999999) };

        assert!(FIXNUM_P(int));
        assert!(!FIXNUM_P(big));
    }
}
