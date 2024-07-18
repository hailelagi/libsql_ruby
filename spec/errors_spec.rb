# frozen_string_literal: true

RSpec.describe "matching error message with string" do
  it "matches the error message" do
    expect { LibSQL.hello_raise("life the universe") }
     # .to raise_error(LibSQL::InvalidUTF8Path, "path has invalid UTF-8")
          # .to raise_error(LibSQL::InvalidUTF8Path, "path has invalid UTF-8")
  end
end
