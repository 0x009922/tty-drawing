defmodule TTX.CLI do
  @spec main(any) :: no_return
  def main(_args) do
    TTX.Dots.create("Hello")
    |> loop()
  end

  @spec loop(TTX.Dots.t()) :: no_return
  defp loop(dots) do
    value = TTX.Dots.get_draw_string(dots)
    clear = IO.ANSI.clear_line()
    cursor_left = IO.ANSI.cursor_left(999)
    IO.write(clear <> cursor_left <> value)
    Process.sleep(50)
    loop(dots)
  end
end
