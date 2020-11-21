defmodule TTX.Tools.Bench do
  def run do
    elems = [
      TTX.Components.Root.new()
    ]
    {rows, cols} = {100, 100}

    Benchee.run(
      %{
        "render" => fn -> TTX.Composition.Builder.build(elems, rows, cols) end,
      }
    )
  end
end
