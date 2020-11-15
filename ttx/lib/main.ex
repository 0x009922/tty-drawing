defmodule TTX.CLI do
  # alias TTX.Dots, as: TextDots
  alias TTX.Composition.{Dot, Builder, Offset}

  @spec main(any) :: no_return
  def main(_args) do
    # alias TTX.Composition.Window

    # alias TTX.Composition.Offset
    # alias TTX.Composition.Component
    # alias TTX.Composition

    elems = [
      Offset.new(2, 2, [
        TTX.TextDotsComponent.new("Looking at eternity")
      ])
    ]

    loop(elems)

    # Builder.build(elems, 10, 10)
    # |> IO.puts()

    # Composition.unwrap_components([
    #   Window.new(5, 3, 10, 10, [
    #     Dot.new(0, 0, "Ola")
    #   ]),
    #   Offset.new(5, 3, []),
    #   Dot.new(5, 3, "hello!"),
    #   %TTX.Wrapper{
    #     elems: [
    #       Dot.new(0, 0, "wrapped!")
    #     ]
    #   }
    #   # Window.new(5, 3, 10, 10, []),
    # ])
    # |> IO.inspect()

    # init_state() |> loop()
  end

  def loop(elems) do
    rendered = Builder.build(elems, 10, 30)
    IO.write(IO.ANSI.clear() <> IO.ANSI.cursor(0, 0))
    IO.write(rendered)

    Process.sleep(100)
    loop(elems)
  end

  # defp init_state() do
  #   {:ok, agent} = Agent.start_link(fn -> nil end)

  #   # Заполняю данными
  #   Agent.update(agent, fn _nil ->
  #     0..5
  #     |> Enum.map(fn key ->
  #       aptext = create_aptext(fn -> exit_fn_for_key(key, agent) end)
  #       {key, aptext}
  #     end)
  #     |> Enum.into(%{})
  #   end)

  #   agent
  # end

  # defp exit_fn_for_key(key, agent) do
  #   Agent.update(agent, fn map ->
  #     Map.update!(map, key, fn _current ->
  #       create_aptext(fn -> exit_fn_for_key(key, agent) end)
  #     end)
  #   end)
  # end

  # defp create_aptext(exit_fn) do
  #   ApText.init(random_text(), exit_fn)
  # end

  # defp inspect_state(agent) do
  #   map = Agent.get(agent, & &1)

  #   IO.write(IO.ANSI.clear() <> IO.ANSI.cursor(0, 0))
  #   # IO.puts("\nInspection")

  #   Enum.each(map, fn {_, v} ->
  #     value = ApText.current_text(v)
  #     IO.puts("#{value}")
  #   end)
  # end

  # defp loop(agent) do
  #   Process.sleep(50)
  #   inspect_state(agent)
  #   loop(agent)
  # end

  # defp random_text() do
  #   x = floor(:rand.uniform(@text_samples_count - 1))
  #   Enum.fetch!(@text_samples, x)
  # end
end
