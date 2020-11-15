alias TTX.Dots, as: TextDots
alias TTX.Composition.{Component, Dot}

defmodule TTX.TextDotsComponent do
  use TypedStruct

  typedstruct enforce: true do
    field(:dots, TextDots.t())
  end

  def new(text) do
    txt_dots = TextDots.create(text)
    %__MODULE__{dots: txt_dots}
  end
end

defimpl Component, for: TTX.TextDotsComponent do
  def elements(%TTX.TextDotsComponent{dots: dots}) do
    str = TextDots.get_draw_string(dots)
    Dot.string_to_dots(str)
  end
end
