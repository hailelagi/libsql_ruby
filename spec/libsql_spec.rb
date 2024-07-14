# frozen_string_literal: true

RSpec.describe LibSQL do
  it "has a version number" do
    expect(LibSQL::VERSION).not_to be nil
  end

  it "does something useful" do
    expect(LibSQL.hello("life the universe")).to eq("Hello from Rust, life the universe!")
  end
end
