defmodule Mix.Tasks.Bench do
  use Mix.Task

  def run(_) do
    elems = [
      TTX.Components.Root.new()
    ]

    {rows, cols} = {100, 100}

    Benchee.run(%{
      "render" => fn -> TTX.Composition.Builder.build(elems, rows, cols) end
    })
  end
end
