# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"
require "rake/extensiontask"
require "rubocop/rake_task"
require "rake/tasklib"

RSpec::Core::RakeTask.new(:spec)

begin
  require "ruby_memcheck"
  require "ruby_memcheck/rspec/rake_task"

  namespace :spec do
    RubyMemcheck::RSpec::RakeTask.new(:valgrind)
  end
rescue LoadError => e
  warn("ruby memcheck not available, cannot run valgrind #{e}")
end

RuboCop::RakeTask.new

namespace :rust do
  desc "Build the Rust library in dev mode"
  task :build do
    sh "cargo build"
  end

  desc "Build the Rust library"
  task :release do
    sh "cargo build --release"
  end

  desc "Generate C bindings with cbindgen"
  task :bindings do
    sh "cbindgen --crate libsql-ruby --output ./ext/libsql/bindings.h"
  end
end

PLATFORMS = %w[
  aarch64-linux-gnu
  aarch64-linux-musl
  arm-linux-gnu
  arm-linux-musl
  arm64-darwin
  x64-mingw-ucrt
  x64-mingw32
  x86-linux-gnu
  x86-linux-musl
  x86-mingw32
  x86_64-darwin
  x86_64-linux-gnu
  x86_64-linux-musl
].freeze

task "gem:native" do
  require "rake_compiler_dock"
  sh "bundle package --all"
  PLATFORMS.each do |plat|
    RakeCompilerDock.sh "bundle --local && rake native:#{plat} gem", platform: plat
  end
  RakeCompilerDock.sh "bundle --local && rake java gem", rubyvm: :jruby
end

namespace :extension do
  desc "Build the Ruby extension"
  task build: ["rust:build", "rust:bindings", "libsql"]
end

Rake::ExtensionTask.new("libsql") do |ext|
  ext.ext_dir = "ext/libsql"
  ext.lib_dir = "ext/libsql"
  ext.cross_compile = true
  ext.cross_platform = %w[x86-mingw32 x64-mingw-ucrt x64-mingw32 x86-linux x86_64-linux x86_64-darwin arm64-darwin]
  ext.cross_platform = %w[x86_64-linux arm64-darwin]
end

task default: %i[extension:build spec]
