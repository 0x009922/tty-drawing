defmodule Mix.Tasks.Show do
  use Mix.Task
  alias TTX.Composition.{Builder}

  @spec run(any) :: no_return
  def run(_) do
    elems = [
      TTX.Components.Root.new()
    ]

    {rows, cols} = TTX.Terminal.size()

    loop(elems, rows, cols)
  end

  defp loop(elems, rows, cols) do
    rendered = Builder.build(elems, rows, cols)
    IO.write(IO.ANSI.clear() <> IO.ANSI.cursor(0, 0))
    IO.write(rendered)

    Process.sleep(30)
    loop(elems, rows, cols)
  end
end
