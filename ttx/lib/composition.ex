defmodule TTX.Composition do
  alias TTX.Composition.Offset
  alias TTX.Composition.Window
  alias TTX.Composition.Dot
  alias TTX.Composition.Component

  @type unwraped_comp_elem() :: Offset.t() | Window.t() | Dot.t()
  @type composition_elem() :: unwraped_comp_elem() | any()
  @type composition() :: list(composition_elem())

  @spec unwrap_elements(composition()) :: list(unwraped_comp_elem())
  def unwrap_elements(elems) do
    Enum.map(elems, &unwrap_elem/1)
  end

  def unwrap_elem(%Dot{} = x), do: x
  def unwrap_elem(%Window{} = x), do: x
  def unwrap_elem(%Offset{} = x), do: x

  def unwrap_elem(component) do
    Component.elements(component)
  end
end
