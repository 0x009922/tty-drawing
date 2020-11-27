defmodule TTX.Composition.Builder.Frame.Test do
  use ExUnit.Case
  alias TTX.Composition.Builder.Frame

  def frame(x, y, w, h) do
    Frame.create(x, y, w, h)
  end

  test "Нет пересечения" do
    a = frame(0, 0, 5, 5)
    b = frame(6, 6, 3, 0)

    assert Frame.intersection(a, b) == nil
  end

  test "Внутреннее пересечение" do
    a = frame(0, 0, 10, 10)
    b = frame(3, 3, 5, 5)

    assert Frame.intersection(a, b) == b
  end

  test "OX" do
    a = frame(4, 4, 4, 4)
    b = frame(2, 4, 4, 4)

    assert Frame.intersection(a, b) == frame(4, 4, 2, 4)
  end

  test "OY" do
    a = frame(4, 4, 4, 4)
    b = frame(4, 6, 4, 4)

    assert Frame.intersection(a, b) == frame(4, 6, 4, 2)
  end

  test "Коммутативность" do
    a = frame(0, 0, 10, 10)
    b = frame(3, 3, 10, 10)

    assert Frame.intersection(a, b) == Frame.intersection(b, a)
  end

  test "Относительный фрейм к абсолютному" do
    parent = frame(5, 5, 4, 6)
    child = frame(3, 10, 0, 4)
    expected = frame(8, 15, 0, 4)

    assert Frame.add_coords(child, parent) == expected
  end
end
