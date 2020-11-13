defmodule TTX.Dots do
  use TypedStruct

  typedstruct do
    field :agent_pid, pid(), enforce: true
  end

  @spec create :: t
  def create do
    {:ok, agent} = Agent.start_link(fn -> %{counter: 0} end)
    spawn_link fn -> update_loop(agent) end
    %TTX.Dots{agent_pid: agent}
  end

  defp update_loop(agent) do
    IO.puts("Updating counter")
    Agent.update(agent, fn v ->
      Map.update!(v, :counter, fn x -> x + 1 end)
    end)
    Process.sleep(300)
    update_loop(agent)
  end

  @spec get_draw_string(t) :: number()
  def get_draw_string(dots) do
    dots.agent_pid
    |> Agent.get(fn v -> v.counter end)
  end
end
