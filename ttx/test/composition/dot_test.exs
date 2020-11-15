defmodule TTX.Composition.Dot.Test do
  use ExUnit.Case
  alias TTX.Composition.Dot

  test "empty string to dots" do
    result = Dot.string_to_dots("")

    assert result == []
  end

  test "non empty string to dots" do
    result = Dot.string_to_dots("Myaw")

    expected = [
      Dot.new(0, 0, "M"),
      Dot.new(1, 0, "y"),
      Dot.new(2, 0, "a"),
      Dot.new(3, 0, "w")
    ]

    assert result == expected
  end
end
