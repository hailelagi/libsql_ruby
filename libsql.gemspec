# frozen_string_literal: true

Gem::Specification.new do |s|
  s.name = "libsql-ruby"
  s.version = "0.0.1"
  s.summary = "Ruby driver for libSQL."
  s.description = "Ruby driver for libSQL."
  s.authors = ["Haile Lagi"]
  s.licenses = ["MIT"]
  s.homepage = "https://github.com/hailelagi/libsql-ruby"
  s.required_ruby_version = Gem::Requirement.new(">= 3.0")
  s.rdoc_options = ["README.md"]
  s.extensions << "ext/libsql/extconf.rb"
end
