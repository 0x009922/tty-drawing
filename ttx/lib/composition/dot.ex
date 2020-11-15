defmodule TTX.Composition.Dot do
  use TypedStruct

  typedstruct enforce: true do
    field(:x, integer())
    field(:y, integer())
    field(:val, binary())
  end

  @spec new(integer(), integer(), binary()) :: t()
  def new(x, y, val) do
    %TTX.Composition.Dot{
      x: x,
      y: y,
      val: val
    }
  end

  def string_to_dots(str) do
    String.codepoints(str)
    |> Stream.with_index()
    |> Enum.map(fn {char, index} -> new(index, 0, char) end)
  end
end
