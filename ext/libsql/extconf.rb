# frozen_string_literal: true

require "mkmf"

have_library("c")
have_header("stdio.h")
have_header("bindings.h")

create_makefile "libsql/libsql"
