defmodule TTX.CLI do
  @spec main(any) :: no_return
  def main(_args) do
    dots = TTX.Dots.create()
    loop(dots)
  end

  @spec loop(TTX.Dots.t()) :: no_return
  defp loop(dots) do
    Process.sleep(500)
    value = TTX.Dots.get_draw_string(dots)
    IO.puts("Dots counter (main): #{value}")
    loop(dots)
  end
end
