# frozen_string_literal: true

RSpec.describe LibSQL do
  it "can create a local connection and persists an empty sqlite file" do
    db =  LibSQL::Database.new(url: "mytest.db")

    expect(db.instance_of?(LibSQL::Database))
    expect(db).not_to be nil
  ensure
    db.close
  end

  it "can query a simple expression" do
    db = LibSQL::Database.new

    expect(db.instance_of?(LibSQL::Database))
    expect(db).not_to be nil

    expect(db.add).to eq 43
  ensure
    db.close
  end

  it "does not reuse a dropped/hanging conn" do
    db = LibSQL::Database.new

    expect(db.instance_of?(LibSQL::Database))
    expect(db).not_to be nil
    db.close

    expect { db.add }.to raise_error(IOError, "Connection not available")
  end

  it "executes a simple query and yields rows" do
    db = LibSQL::Database.new

    expect(db.instance_of?(LibSQL::Database))
    expect(db).not_to be nil
    db.close

    rows = db.execute <<-SQL
    create table users (
    name varchar(30),
    age int);
    SQL

    expect rows.to eq nil

    db.execute("INSERT INTO users (name, age) VALUES (?, ?)", ["John", 20])

    db.execute("select * from users") do |row|
      expect(row).to be_a(LibSQL::Row)
      expect(row.name).to eq "John"
      expect(row.age).to eq 20
    end
  end
end
