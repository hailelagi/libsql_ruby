# frozen_string_literal: true

require "mkmf"

have_library("c")
have_header("stdio.h")
have_header("stdbool.h")
have_header("stdint.h")
have_header("stdlib.h")
have_header("stdarg.h")
have_header("bindings.h")

# TODO: replace this with something that can actually build properly
dir = File.expand_path(File.join(__dir__, "../../target/release"))
rust_binary = File.join(dir, "liblibsql_ruby.a")

abort "libsql bin not found: #{rust_binary}" unless File.exist?(rust_binary)

$LDFLAGS << " -L#{dir} -llibsql_ruby" # rubocop:disable Style/GlobalVars

create_makefile "libsql/libsql"
