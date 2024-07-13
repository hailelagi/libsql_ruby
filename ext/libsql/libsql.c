#include <ruby.h>
#include "bindings.h"

#ifndef LIBSQL_INT
#error LibSQL bindings are not generated correctly or there was a problem linking
#endif


void Init_libsql(void)
{    
VALUE libSQL;
   libSQLm = rb_const_get(rb_cObject, rb_intern("LibSQL"));
   libSQL = rb_define_class_under(libSQL, "LibSQL", rb_cObject);

   //  ruby -Ilib:ext -r libsql -e "p LibSQL::Database.new().life_the_universe"
   rb_define_method(libSQL, "life_the_universe", life_the_universe, 0);
   rb_define_method(libSQL, "side_effect", side_effect, 0);
}
