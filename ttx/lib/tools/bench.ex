defmodule TTX.Tools.Bench do
  def run do
    elems = [
      TTX.Components.Root.new()
    ]
    {rows, cols} = {50, 50}

    Benchee.run(
      %{
        "render" => fn -> TTX.Composition.Builder.build(elems, rows, cols) end,
      }
    )
  end
end
