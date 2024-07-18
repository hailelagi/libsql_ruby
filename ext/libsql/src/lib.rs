use ::libsql as libsql_core;
use magnus::{function, prelude::*, Error, ExceptionClass, Module, Ruby};

mod database;
mod errors;

fn hello(subject: String) -> Result<String, Error> {
    Ok(format!("Hello from Rust, {subject}!"))
}

fn hello_raise(subject: String) -> Result<String, Error> {
    Err(errors::to_rb_exception(libsql_core::Error::InvalidUTF8Path))
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("LibSQL")?;
    let except_class = module
        .define_class("Error", ruby.class_object())
        .unwrap();

    except_class.define_error("InvalidUTF8Path", ruby.exception_standard_error());

    module.define_singleton_method("hello", function!(hello, 1))?;
    module.define_singleton_method("hello_raise", function!(hello_raise, 1))?;
    Ok(())
}
