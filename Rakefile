# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"
require "rubocop/rake_task"
require "rb_sys/extensiontask"

RSpec::Core::RakeTask.new(:spec)
RuboCop::RakeTask.new

GEMSPEC = Gem::Specification.load("libsql.gemspec")

RbSys::ExtensionTask.new("libsql", GEMSPEC) do |ext|
  ext.lib_dir = "lib/libsql"
end

begin
  require "ruby_memcheck"
  require "ruby_memcheck/rspec/rake_task"

  namespace :spec do
    RubyMemcheck::RSpec::RakeTask.new(:valgrind)
  end
rescue LoadError => e
  warn("ruby memcheck not available, cannot run valgrind #{e}")
end

desc "Compile the extension with debug symbols"
task "compile:debug" do
  ENV["RB_SYS_CARGO_PROFILE"] = "dev"
  Rake::Task["compile"].invoke
end

task build: :compile
task default: %i[compile spec rubocop]
