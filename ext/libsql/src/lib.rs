use ::libsql;
use database::Database;
use magnus::{function, method, prelude::*, Error, Module, Object, Ruby};

pub mod connection;
mod database;
mod errors;
mod nogvl;

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("LibSQL")?;
    let db = module.define_class("Database", ruby.class_object())?;
    let except_class = module.define_class("Exception", ruby.class_object())?;

    module.const_set("SQLITE3_VERSION", libsql::version())?;

    except_class.define_error("InvalidUTF8Path", ruby.exception_standard_error())?;

    db.define_alloc_func::<database::Database>();
    db.define_method("initialize", method!(database::Database::initialize, -1))?;

    // module.define_private_method("bye", method!(database::world, 1))?;
    // module.define_singleton_method("hello_raise", function!(database::hello_raise, 1))?;

    Ok(())
}
