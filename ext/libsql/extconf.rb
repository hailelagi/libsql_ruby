# frozen_string_literal: true

require "mkmf"

have_library("c")
have_header("stdio.h")
have_header("bindings.h")

dir = File.expand_path(File.join(__dir__, "../../target/release"))
libsql = "libsql_ruby"

rust_binary = File.join(dir, "lib#{libsql}.a")
abort "libsql bin not found: #{rust_binary}" unless File.exist?(rust_binary)

$LDFLAGS << " -L#{dir} -l#{libsql}" # rubocop:disable Style/GlobalVars

create_makefile "libsql/libsql"
