# frozen_string_literal: true

# standard error classes
module LibSQL
  # errors
  class Error < StandardError; end

  class ConnectionFailed < Error; end

  class SqliteFailure < Error; end

  class NullValue < Error; end

  class Misuse < Error; end

  class ExecuteReturnedRows < Error; end

  class QueryReturnedNoRows < Error; end

  class InvalidColumnName < Error; end

  class ToSqlConversionFailure < Error; end

  class SyncNotSupported < Error; end

  class LoadExtensionNotSupported < Error; end

  class ColumnNotFound < Error; end

  class Hrana < Error; end

  class WriteDelegation < Error; end

  class Bincode < Error; end

  class InvalidColumnIndex < Error; end

  class InvalidColumnType < Error; end

  class Sqlite3SyntaxError < Error; end

  class Sqlite3UnsupportedStatement < Error; end

  class Sqlite3ParserError < Error; end

  class RemoteSqliteFailure < Error; end

  class Replication < Error; end

  class InvalidUTF8Path < Error; end

  class FreezeNotSupported < Error; end

  class InvalidParserState < Error; end

  class InvalidTlsConfiguration < Error; end

  class TransactionalBatchError < Error; end

  class InvalidBlobSize < Error; end
end
