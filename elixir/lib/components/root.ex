defmodule TTX.Components.Root do
  use TypedStruct
  alias TTX.Composition.Component
  alias TTX.Components.Artifacts

  typedstruct enforce: true do
    field(:children, list())
  end

  @spec new :: t()
  def new do
    {rows, cols} = TTX.Terminal.size()

    %__MODULE__{
      children: [
        Artifacts.new(rows, cols)
      ]
    }
  end

  defimpl Component, for: __MODULE__ do
    def elements(self), do: self.children
  end
end
