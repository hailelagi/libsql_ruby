# frozen_string_literal: true

lib = File.expand_path("lib", __dir__)
$LOAD_PATH.unshift(lib) unless $LOAD_PATH.include?(lib)
require "libsql"

Gem::Specification.new do |s|
  s.name = "Libsql"
  s.version = LibSQL::VERSION
  s.summary = "Ruby driver for libSQL."
  s.description = "Ruby driver for libSQL."
  s.authors = ["Haile Lagi"]
  s.licenses = ["MIT"]
  s.homepage = "https://github.com/hailelagi/libsql-ruby"
  s.required_ruby_version = Gem::Requirement.new(">= 3.0")

  s.files = [
    "LICENSE",
    "README.md",
    "ext/libsql/extconf.rb"
  ]

  s.rdoc_options = ["README.md"]
  s.extensions << "ext/libsql/extconf.rb"
end
