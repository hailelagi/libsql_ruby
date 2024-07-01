require 'mkmf'

# Detect the platform
platform = RbConfig::CONFIG['host_os']
arch = `uname -m`.strip

case platform
when /darwin/
  # Specific configurations for Darwin (MacOS)
  if arch == 'arm64'
    $CFLAGS << ' -target arm64-apple-macos11'
    $LDFLAGS << ' -target arm64-apple-macos11'
  end

  # Path to the Rust library
  rust_lib_path = File.expand_path('../../../libsql-ruby/target/release', __FILE__)
  $LDFLAGS << " -L#{rust_lib_path} -llibsql-ruby"
when /linux/
  rust_lib_path = File.expand_path('../../../libsql-ruby/target/release', __FILE__)
  $LDFLAGS << " -L#{rust_lib_path} -llibsql-ruby"
else
  raise "Unsupported platform: #{platform}"
end

have_library('c')
have_header('stdio.h')
create_makefile('libsql/native')
