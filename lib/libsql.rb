# frozen_string_literal: true

require_relative "libsql/database"

module LibSQL
  class Error < StandardError; end

  def self.threadsafe?
    true
  end
end
