# LibSQL Ruby

A low-level ruby driver for libSQL/SQLite3. You're probably interested in the `activerecord-libsql` adapter which would depend on this.

## Example

```ruby
require "libsql_ruby"

db = LibSQL::Database.new "test.db"

rows = db.execute <<-SQL
  create table user (
    name varchar(30),
    age int
  );
SQL
```

## Installation

Install the gem and add to the application's Gemfile by executing:

    $ bundle add libsql_ruby

If bundler is not being used to manage dependencies, install the gem by executing:

    $ gem install libsql_ruby

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/hailelagi/libsql_ruby.

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
