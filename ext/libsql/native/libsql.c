#include <stdio.h>
#include <ruby.h>

extern char *hello_rust(void);
extern void fill_slice(char *);
extern int num(void);

VALUE hello(void)
{
    char *hello = hello_rust();
    printf("%s\n", hello);

    return Qnil;
}

VALUE make_hello()
{
    // "Hello, world!".length == 14, with the null
    char *hello = (char *)malloc(sizeof(char) * 14);

    fill_slice(hello);

    printf("%s\n", hello);

    free(hello);

    return Qnil;
}

VALUE number()
{
    int x = num();

    return INT2FIX(x);
}


void Init_libsql(void)
{
    VALUE libsql = rb_define_module("LibSQL");

    rb_define_singleton_method(libsql, "hello", hello, 0);
    rb_define_singleton_method(libsql, "make_hello", make_hello, 0);
    rb_define_singleton_method(libsql, "number", number, 0);
}