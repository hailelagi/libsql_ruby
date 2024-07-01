require "mkmf"

platform = RbConfig::CONFIG['host_os']

have_library('c')
have_header('stdio.h')

# abort "missing malloc()" unless have_func "malloc"
# abort "missing free()"   unless have_func "free"

create_makefile "libsql/libsql"
