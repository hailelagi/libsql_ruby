# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"
require 'rake/extensiontask'
require "rubocop/rake_task"
require 'rake/tasklib'

RSpec::Core::RakeTask.new(:spec)
RuboCop::RakeTask.new

namespace :rust do
  desc "Build the Rust library in dev mode"
  task :build do
    sh 'cargo build'
  end

  desc "Build the Rust library"
  task :release do
    sh 'cargo build --release'
  end

  desc "Generate C bindings with cbindgen"
  task :bindings do
    sh 'cbindgen --crate libsql-ruby --output ./ext/libsql/bindings.h'
  end
end

namespace :extension do
  desc "Build the Ruby extension"
  task :build => ['rust:build', 'rust:bindings', 'libsql']
end

Rake::ExtensionTask.new('libsql') do |ext|
  ext.ext_dir = "ext/libsql"
  ext.lib_dir = 'ext/libsql'
end

task default: %i[extension:build spec]
