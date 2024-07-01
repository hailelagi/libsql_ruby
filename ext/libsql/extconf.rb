require "mkmf"

platform = RbConfig::CONFIG['host_os']

case platform
when /darwin/
  arch = `uname -m`.strip
  if arch == 'arm64'
    $CFLAGS << ' -target arm64-apple-macos11'
    $LDFLAGS << ' -target arm64-apple-macos11'
  elsif arch == 'x86_64'
    $CFLAGS << ' -target x86_64-apple-macos10.15'
    $LDFLAGS << ' -target x86_64-apple-macos10.15'
  end
when /linux/
  arch = `uname -m`.strip
  if arch == 'x86_64'
  raise "Unsupported: planned support soon! for #{platform}"
  elsif arch == 'arm64'
  raise "Unsupported: planned support soon! for #{platform}"
  end
else
  raise "Unsupported platform please raise an issue: #{platform}"
end

have_library('c')
have_header('stdio.h')

# abort "missing malloc()" unless have_func "malloc"
# abort "missing free()"   unless have_func "free"

create_makefile "libsql/libsql"