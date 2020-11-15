defmodule TTX.Composition.Builder.Buffer.Test do
  use ExUnit.Case
  alias TTX.Composition.Builder.Buffer

  test "Пустая строчка из пробелов" do
    result =
      Buffer.new(1, 5)
      |> Buffer.build()

    assert result == "     "
  end

  test "Две строки из пробелов, разделённые переносом" do
    result =
      Buffer.new(2, 2)
      |> Buffer.build()

    assert result == "  \n  "
  end

  test "Рендерит символ" do
    result =
      Buffer.new(2, 2)
      |> Buffer.write(0, 0, "x")
      |> Buffer.build()

    assert result == "x \n  "
  end

  test "Не рисует то, что за границами" do
    result =
      Buffer.new(4, 4)
      |> Buffer.write(0, 0, "h")
      |> Buffer.write(1, 0, "e")
      |> Buffer.write(2, 0, "l")
      |> Buffer.write(3, 0, "l")
      |> Buffer.write(4, 0, "o")
      |> Buffer.write(0, 1, "e")
      |> Buffer.write(0, 2, "l")
      |> Buffer.write(0, 3, "l")
      |> Buffer.write(0, 4, "o")
      |> Buffer.build()

    assert result == "hell\ne   \nl   \nl   "
  end
end
