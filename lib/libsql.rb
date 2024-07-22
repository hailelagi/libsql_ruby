# frozen_string_literal: true

require_relative "libsql/version"
require_relative "libsql/errors"
require_relative "libsql/pragma"

begin
  RUBY_VERSION =~ /(\d+\.\d+)/
  require "libsql/#{Regexp.last_match(1)}/libsql"
rescue LoadError
  require "libsql/libsql"
end

module LibSQL
  class Database
    include Pragmas
  end
end
