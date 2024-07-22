use ::libsql as libsql_core;
use magnus::{eval, Error, ExceptionClass};

// TODO: could refactor with a lazy static init by pre-processing the Enums
/*
https://github.com/matsadler/magnus/blob/main/examples/custom_exception_ruby/ext/ahriman/src/lib.rs#L6 &
https://github.com/matsadler/magnus/blob/main/examples/custom_exception_rust/ext/ahriman/src/lib.rs
static ERROR: Lazy<ExceptionClass> = Lazy::new(|ruby| {
    ruby
        .get_inner(&AHRIMAN)
        .define_error("Error", ruby.exception_standard_error())
        .unwrap()
});

*/
pub fn to_rb_exception(error: libsql_core::errors::Error) -> Error {
    let message = match &error {
        libsql_core::Error::SqliteFailure(_, err) => err.to_string(),
        _ => error.to_string(),
    };

    let except_klass = ExceptionClass::from_value(eval("StandardError").unwrap()).unwrap();

    Error::new(except_klass, message)
}

#[cfg(test)]
mod tests {
    use super::*;
    use magnus::{eval, typed_data::Inspect, ExceptionClass};

    use rb_sys_test_helpers::ruby_test;

    #[ruby_test]
    fn test_standard_exceptions_raise() {
        assert!(ExceptionClass::from_value(eval("StandardError").unwrap()).is_some());
        assert!(
            ExceptionClass::from_value(eval(r#"StandardError.new("example")"#).unwrap()).is_none()
        );
    }

    #[ruby_test]
    fn test_invalid_utf8() {
        let err_inspect = "Error(Error(StandardError, \"path has invalid UTF-8\"))";

        assert_eq!(
            err_inspect,
            to_rb_exception(libsql_core::Error::InvalidUTF8Path).inspect()
        );
    }
}
