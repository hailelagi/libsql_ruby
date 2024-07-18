# frozen_string_literal: true

# standard error classes
module LibSQL
  # errors
  class Exception < StandardError; end

  class ConnectionFailed < Exception; end

  class SqliteFailure < Exception; end

  class NullValue < Exception; end

  class Misuse < Exception; end

  class ExecuteReturnedRows < Exception; end

  class QueryReturnedNoRows < Exception; end

  class InvalidColumnName < Exception; end

  class ToSqlConversionFailure < Exception; end

  class SyncNotSupported < Exception; end

  class LoadExtensionNotSupported < Exception; end

  class ColumnNotFound < Exception; end

  class Hrana < Exception; end

  class WriteDelegation < Exception; end

  class Bincode < Exception; end

  class InvalidColumnIndex < Exception; end

  class InvalidColumnType < Exception; end

  class Sqlite3SyntaxError < Exception; end

  class Sqlite3UnsupportedStatement < Exception; end

  class Sqlite3ParserError < Exception; end

  class RemoteSqliteFailure < Exception; end

  class Replication < Exception; end

  class InvalidUTF8Path < Exception; end

  class FreezeNotSupported < Exception; end

  class InvalidParserState < Exception; end

  class InvalidTlsConfiguration < Exception; end

  class TransactionalBatchError < Exception; end

  class InvalidBlobSize < Exception; end
end
