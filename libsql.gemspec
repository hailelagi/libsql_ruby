# frozen_string_literal: true

require_relative "lib/libSQL/version"

Gem::Specification.new do |spec|
  spec.name = "libSQL"
  spec.version = LibSQL::VERSION
  spec.authors = ["Haile Lagi"]
  spec.email = ["52631736+hailelagi@users.noreply.github.com"]

  spec.summary = "Ruby driver for libSQL."
  spec.homepage = "https://github.com/hailelagi/libsql_ruby"
  spec.license = "MIT"
  spec.required_ruby_version = ">= 3.0.0"
  spec.required_rubygems_version = ">= 3.3.11"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/hailelagi/libsql_ruby"
  spec.metadata["changelog_uri"] = "https://github.com/hailelagi/libsql_ruby/CHANGELOG.md"

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  gemspec = File.basename(__FILE__)
  spec.files = IO.popen(%w[git ls-files -z], chdir: __dir__, err: IO::NULL) do |ls|
    ls.readlines("\x0", chomp: true).reject do |f|
      (f == gemspec) ||
        f.start_with?(*%w[bin/ test/ spec/ features/ .git .github appveyor Gemfile])
    end
  end
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/libSQL/Cargo.toml"]

  spec.add_dependency "rubocop", "~> 1.59.0"
  spec.add_dependency "ruby_memcheck", "3.0.0" if Gem::Platform.local.os == "linux"

  # For more information and examples about making a new gem, check out our
  # guide at: https://bundler.io/guides/creating_gem.html
end
