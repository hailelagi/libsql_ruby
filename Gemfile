# frozen_string_literal: true

source "https://rubygems.org"

# Specify your gem's dependencies in libsql.gemspec
gemspec

gem "rake", "~> 13.0"
gem "rake-compiler"
gem "rb_sys", "~> 0.9.63"
gem "rspec", "~> 3.0"
gem "rubocop", "~> 1.21", require: false, group: :development
gem "ruby_memcheck", "3.0.0" if Gem::Platform.local.os == "linux"
gem "sorbet", group: :development
gem "sorbet-runtime"
gem "tapioca", require: false, group: %i[development test]
