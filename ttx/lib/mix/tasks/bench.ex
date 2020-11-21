defmodule Mix.Tasks.Bench do
  use Mix.Task

  def run(_) do
    TTX.Tools.Bench.run()
    # IO.puts("Hi!")
  end
end
