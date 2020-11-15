defmodule TTX.Wrapper do
  defstruct elems: nil
end

defimpl TTX.Composition.Component, for: TTX.Wrapper do
  def elements(wrapper), do: wrapper.elems
end
