defmodule TTX.Components.Artifacts do
  alias TTX.Components.ArtifactItem
  alias TTX.Composition.Component
  alias TTX.Tools.Ease

  @moduledoc """
  Компонент реализует отрисовку артефактов.

  Управляет всеми имеющимися частицами. Периодически волнообразно их обновляет.
  """

  use TypedStruct

  typedstruct enforce: true do
    field :items, [{float(), ArtifactItem.t()}]
  end

  def new(rows, cols) do
    # {rows, cols} = TTX.Terminal.size()
    # {rows, cols} =

    cx = cols / 2
    cy = rows / 2
    radius = piph_terminal(cx, cy)

    items =
      Enum.reduce(0..(rows - 1), [], fn y, acc ->
        Enum.reduce(0..(cols - 1), acc, fn x, acc ->
          item_rad = piph_terminal(cx - x, cy - y)
          item_val = item_rad / radius
          # IO.puts("(#{x}; #{y}) #{item_rad} / #{radius} = #{item_val}")

          item = ArtifactItem.new(%{
            x: x,
            y: y,
            remoteness: item_val
          })

          [{item_val, item} | acc]
        end)
      end)
      |> Enum.sort(fn {a, _}, {b, _} -> a <= b end)
      # |> Enum.map(fn {_, item} -> item end)

    self = %__MODULE__{items: items}
    spawn_link fn -> upd_loop(self) end

    self




    # надо делать сетку на весь viewport
    # всю её забить артефакт-итемами
    # далее каждый положить в карту по особому ключу, говорящему об удалённости точки
    # и потом, раз в n секунд, запускать апдейт итемов волнообразно от центра
    # как?
  end

  @spec upd_loop(t()) :: no_return
  def upd_loop(self) do
    Ease.run(1300, &ease_callback/2, state: self.items, ticks_per_second: 30, timing_fn: &ease_timing/1)
    Process.sleep(700)
    upd_loop(self)
  end

  defp ease_timing(x) do
    :math.sin(x * :math.pi / 2)
  end

  defp ease_callback(items, time) do
    fire_updates(items, time)
    # IO.inspect(Enum.take(items, 2))
    # IO.puts("Tick! #{time}")
    # items
  end

  defp fire_updates(items, time)
  defp fire_updates([], _), do: []
  defp fire_updates([{val, _} | _] = items, time) when val > time, do: items
  defp fire_updates([{_, artifact} | tail], time) do
    ArtifactItem.update(artifact)
    fire_updates(tail, time)
  end

  defp piph_terminal(x, y), do: piph(x, y * 2.1)

  defp piph(a, b) do
    (a * a) + (b * b)
    |> :math.pow(0.5)
  end

  defimpl Component, for: __MODULE__ do
    def elements(%TTX.Components.Artifacts{items: items}) do
      Enum.map(items, fn {_, x} -> x end)
      # IO.puts("artifacts")
      # Enum.map(items, fn {_, x} -> x end)
      # |> Task.async_stream(fn item -> Component.elements(item) end)
      # |> Enum.map(fn {:ok, [val]} -> val end)
      # |> Enum.to_list()
    end
  end
end
