use ::libsql;
use magnus::{method, prelude::*, Error, Module, Ruby};

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
    db.define_method("close", method!(database::Database::close, 0))?;
    db.define_method("add", method!(database::Database::expr, 0))?;
    db.define_method("execute", method!(database::Database::execute, 2))?;

    // module.define_private_method("bye", method!(database::world, 1))?;
    // module.define_singleton_method("hello_raise", function!(database::hello_raise, 1))?;

    Ok(())
}
