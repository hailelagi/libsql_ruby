# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"
require 'rake/extensiontask'
require "rubocop/rake_task"
require 'rake/tasklib'

RSpec::Core::RakeTask.new(:spec)
RuboCop::RakeTask.new

namespace :rust do
  desc "Build the Rust library"
  task :build do
    sh 'cargo build --release --manifest-path=libsql-ruby/Cargo.toml'
  end

  desc "Generate C bindings with cbindgen"
  task :bindings do
    sh 'cbindgen --config libsql-ruby/cbindgen.toml --crate libsql-ruby --output libsql-ruby/ext/libsql/bindings.h'
  end
end

namespace :extension do
  desc "Build the Ruby extension"
  task :build => ['rust:build', 'rust:bindings', 'build-ext']
end

Rake::ExtensionTask.new('build-ext') do |ext|
  ext.ext_dir = "ext/libsql"
  ext.lib_dir = 'ext/libsql'
end

task default: %i[spec extension:build]