defmodule TTX.Composition.Offset do
  use TypedStruct

  typedstruct enforce: true do
    field(:x, integer())
    field(:y, integer())
    field(:elements, list())
  end

  @spec new(integer(), integer(), list()) :: t()
  def new(x, y, elems) do
    %TTX.Composition.Offset{
      x: x,
      y: y,
      elements: elems
    }
  end
end
