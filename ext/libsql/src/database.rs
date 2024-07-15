//! Database.rs 
//! 
//! 

#[cfg(test)]
mod tests {
    use rb_sys_test_helpers::ruby_test;
    use rb_sys::{rb_num2fix, rb_int2big, FIXNUM_P};

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