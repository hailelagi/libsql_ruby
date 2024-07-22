//! Database.rs

use std::cell::RefCell;
use std::sync::{Arc, Mutex};

use magnus::typed_data::Obj;
use magnus::{
    scan_args::{get_kwargs, scan_args},
    typed_data, Error, Value,
};

use crate::connection::runtime::TOKIO_RT;

#[derive(Default)]
#[magnus::wrap(class = "LibSQL::Database")]
pub(crate) struct Database {
    conn: RefCell<Option<Arc<Mutex<libsql::Connection>>>>,
    db: RefCell<Option<Arc<Mutex<libsql::Database>>>>,
}

impl Database {
    pub fn initialize(
        rb_self: typed_data::Obj<Self>,
        args: &[Value],
    ) -> Result<Obj<Database>, Error> {
        let args = scan_args::<(), (), (), (), _, ()>(args)?;
        let args = get_kwargs(args.keywords, &[], &["url"])?;

        let _: () = args.required;
        let (url,): (Option<String>,) = args.optional;
        let _: () = args.splat;
        let url = url.unwrap_or_else(|| String::from("test.db"));

        let db = TOKIO_RT
            .block_on(async { libsql::Builder::new_local(url).build().await })
            .unwrap();
        let conn = TOKIO_RT.block_on(async { db.connect() }).unwrap();

        rb_self.db.borrow_mut().replace(Arc::new(Mutex::new(db)));
        rb_self
            .conn
            .borrow_mut()
            .replace(Arc::new(Mutex::new(conn)));

        Ok(rb_self)
    }

    pub fn add(&self) -> Result<i64, magnus::Error> {
        let conn = self
            .conn
            .borrow()
            .as_ref()
            .ok_or(magnus::Error::new(
                magnus::exception::io_error(),
                "Connection not available",
            ))?
            .clone();
        let conn = conn.lock().unwrap();

        let result = TOKIO_RT.block_on(async {
            let mut rows = conn.query("SELECT 1 + ?1", [42]).await.unwrap();

            let row = rows.next().await.unwrap().unwrap();
            let value = row.get_value(0).unwrap();

            *value.as_integer().unwrap()
        });

        Ok(result)
    }

    pub fn close(&self) -> Result<(), Error> {
        let conn = self
            .conn
            .borrow()
            .as_ref()
            .ok_or(magnus::Error::new(
                magnus::exception::io_error(),
                "Connection not available",
            ))?
            .clone();
    
        let conn = conn.lock().unwrap();
        TOKIO_RT.block_on(async { conn.reset().await });

        self.conn.replace(None);

        Ok(())
    }
}
