defmodule TTX.Composition.Window do
  use TypedStruct

  typedstruct enforce: true do
    field(:x, integer())
    field(:y, integer())
    field(:width, non_neg_integer())
    field(:height, non_neg_integer())
    field(:elements, list())
  end

  @spec new(integer(), integer(), integer(), integer(), list()) :: t()
  def new(x, y, w, h, elems) do
    %TTX.Composition.Window{
      x: x,
      y: y,
      width: w,
      height: h,
      elements: elems
    }
  end
end
