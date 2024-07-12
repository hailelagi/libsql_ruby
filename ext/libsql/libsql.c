#include <ruby.h>
#include "bindings.h"

#ifndef LIBSQL_INT
#error LibSQL bindings are not generated correctly or there was a problem linking
#endif


void Init_libsql(void)
{    VALUE libSQL;
    libSQL = rb_define_module("LibSQL");

    libSQL = rb_const_get(rb_cObject, rb_intern("LibSQL"));

}
