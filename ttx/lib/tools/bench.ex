defmodule TTX.Tools.Bench do
  def run do
    elems = [
      TTX.Components.Root.new()
    ]
    {rows, cols} = TTX.Terminal.size()

    Benchee.run(
      %{
        "render" => fn -> TTX.Composition.Builder.build(elems, rows, cols) end
      }
    )
  end
end
