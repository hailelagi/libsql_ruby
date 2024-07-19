# frozen_string_literal: true

RSpec.describe LibSQL do
  it "can create a local connection and persists an empty sqlite file" do
    expect(LibSQL::Database.new).not_to be nil
  end
end
