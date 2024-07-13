RSpec.describe LibSQL::Database do
  it "says hello world via C via Rust" do
    expect(LibSQL::Database.new.life_the_universe).to eq(42)
  end
end
