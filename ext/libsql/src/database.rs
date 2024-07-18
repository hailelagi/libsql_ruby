//! Database.rs
//!
//!
use libsql::{Builder, Database};
use magnus::{function, prelude::*, Error, ExceptionClass, Ruby};


// figure out async?
// fn connect(ruby: &Ruby, database: String) -> Result<(), Error> {
//     let db = Builder::new_local(":memory:").build().await.unwrap();
//     let conn = db.connect().unwrap();

//     Ok(())
// }

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
