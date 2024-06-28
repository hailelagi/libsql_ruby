Gem::Specification.new do |s|
  s.name = "libsql"
  s.version = "0.0.1"
  s.summary = "Ruby interface to libsql."
  s.description = "Ruby interface to libsql."
  s.authors = ["Haile Lagi"]
  s.licenses = ["MIT"]
  s.required_ruby_version = Gem::Requirement.new(">= 3.0")

  s.files = [
    ".gemtest",
    "LICENSE",
    "README.md",
    "dependencies.yml",
    "ext/libsql/extconf.rb",
    "ext/libsql/libsql.c",
  ]

  s.rdoc_options = ["README.md"]
  s.extensions << "ext/extconf.rb"
end
