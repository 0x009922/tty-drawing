defmodule TTX.Composition.Dot do
  use TypedStruct

  @type dot_val() :: binary() | nil

  typedstruct enforce: true do
    field(:x, integer())
    field(:y, integer())
    field(:val, dot_val())
  end

  @spec new(integer(), integer(), dot_val()) :: t()
  def new(x, y, val) do
    %TTX.Composition.Dot{
      x: x,
      y: y,
      val: val
    }
  end

  @spec update_val(t(), dot_val()) :: t()
  def update_val(dot, new_val) do
    Map.replace!(dot, :val, new_val)
  end

  @spec string_to_dots(binary) :: [t()]
  def string_to_dots(str) do
    String.codepoints(str)
    |> Stream.with_index()
    |> Enum.map(fn {char, index} -> new(index, 0, char) end)
  end
end
