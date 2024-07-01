# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"
require 'rake/extensiontask'
require "rubocop/rake_task"

RSpec::Core::RakeTask.new(:spec)
RuboCop::RakeTask.new

task default: %i[spec build]

Rake::ExtensionTask.new('libsql_native') do |ext|
  ext.ext_dir = "ext/libsql"
  ext.lib_dir = 'ext/libsql'
end

