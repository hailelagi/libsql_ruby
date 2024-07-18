//! Database.rs

use std::cell::RefCell;
use std::sync::Mutex;

use libsql;
use libsql::{Builder, Database};
use magnus::{
    function, class, define_class, exception, method, module,
    prelude::*,
    scan_args::{get_kwargs, scan_args},
    typed_data, Error, Value,
};

use crate::errors;

pub enum Mode {
    Remote,
    Local,
    RemoteReplica
}

#[magnus::wrap(class = "LibSQL::Database")]
pub(crate) struct LibSQL {
    mode: Mode,
    conn_id: String,
    db: Option<RefCell<Mutex<libsql::Database>>>
}

impl LibSQL {
    fn initialize(rb_self: typed_data::Obj<Self>, args: &[Value]) -> Result<(), Error> {
        let args = scan_args::<(), (), (), (), _, ()>(args)?;
        let kwargs = get_kwargs::<_, (), (Option<String>, Option<String>, Option<String>), ()>(
            args.keywords,
            &[],
            &["remote", "local", "remote_replica"],
        )?;
    
        Ok(())
    }
    
    // figure out async? - RefCell Mutex it all as everything is wrapped in the GVL anyway
    // so we cannot `await`
    fn connect(rb_self: typed_data::Obj<Self>) -> Result<(), Error> {
    let db = Builder::new_local(":memory:").build();
    let conn = db.connect().unwrap();

    Ok(())
}
}

pub fn hello(subject: String) -> Result<String, Error> {
    Ok(format!("Hello from Rust, {subject}!"))
}

pub fn world(subject: String) -> String {
    format!("Hello from Rust, {subject}!")
}

pub fn hello_raise(subject: String) -> Result<String, Error> {
    Err(errors::to_rb_exception(libsql::Error::InvalidUTF8Path))
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
