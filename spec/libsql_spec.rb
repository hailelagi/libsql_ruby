# frozen_string_literal: true

RSpec.describe LibSQL do
  it "has a version number" do
    expect(LibSQL::VERSION).not_to be nil
  end

  it "does something useful" do
    expect(true).to eq(true)
  end

  it "says hello world via C via Rust" do
     expect(LibSQL.hello()).not_to be nil
  end

  it "says hello world via C" do
     expect(LibSQL.make_hello()).not_to be nil
  end

  it "returns a fixed size type" do
     expect(LibSQL.number()).not_to be nil
  end
end
