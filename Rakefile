# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"

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

require "rubocop/rake_task"

RuboCop::RakeTask.new

require "rb_sys/extensiontask"

task build: :compile

GEMSPEC = Gem::Specification.load("libsql.gemspec")

RbSys::ExtensionTask.new("libsql", GEMSPEC) do |ext|
  ext.lib_dir = "lib/libsql"
end

task default: %i[compile spec rubocop]
