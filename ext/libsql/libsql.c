#include <stdio.h>
#include <ruby.h>
#include "bindings.h"


void Init_libsql(void)
{    VALUE libSQL;

    libSQL = rb_const_get(rb_cObject, rb_intern("LibSQL"));

}
