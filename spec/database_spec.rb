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
end
