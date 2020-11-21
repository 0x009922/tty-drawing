defmodule TTX.Composition.Builder.Test do
  use ExUnit.Case
  alias TTX.Composition.{Builder, Dot, Offset, Window}

  test "Билд пустой композиции" do
    result = Builder.build([], 10, 5)
    expected = Enum.map(1..10, fn _ -> "     " end) |> Enum.join("\n")

    assert result == expected
  end

  test "Есть одна точка" do
    rows = 5
    columns = 5
    elements = [
      Dot.new(2, 3, "*")
    ]

    result = Builder.build(elements, rows, columns)

    assert result == "     \n     \n     \n  *  \n     "
  end

  test "Есть точка в смещении" do
    rows = 5
    columns = 5
    elements = [
      Offset.new(2, 2, [
        Dot.new(1, 2, "Y")
      ])
    ]

    result = Builder.build(elements, rows, columns)

    assert result == "     \n     \n     \n     \n   Y "
  end

  test "Есть точка в смещении в окне, и просто в смещении" do
    rows = 3
    columns = 3
    elements = [
      Offset.new(1, 0, [
        Window.new(1, 1, 10, 10, [
          Dot.new(0, 0, "^")
        ]),
        Dot.new(0, 0, "$")
      ])
    ]

    result = Builder.build(elements, rows, columns)

    assert result == " $ \n  ^\n   "
  end

  test "Точка - nil" do
    result = Builder.build([Dot.new(1, 1, nil)], 3, 3)

    assert result == "   \n   \n   "
  end
end
