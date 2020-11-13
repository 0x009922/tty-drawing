defmodule TTX.Dots do
  use TypedStruct

  typedstruct do
    field :agent_pid, pid(), enforce: true
  end

  defmodule DotsState do
    typedstruct enforce: true do
      field :counter, integer()
      field :source_text, String.t
    end

    @spec create(String.t) :: t
    def create(source) do
      %DotsState{
        counter: 0,
        source_text: source
      }
    end

    @spec update(t) :: t
    def update(val) do
      val |> Map.update!(:counter, &(rem(&1 + 1, 4)))
    end

    @spec compute_text(t) :: String.t
    def compute_text(val) do
      dots = String.duplicate(".", val.counter)
      val.source_text <> dots
    end
  end

  @spec create(text: String.t) :: t
  def create(text) do
    {:ok, agent} = Agent.start_link(fn -> DotsState.create(text) end)
    spawn_link fn -> update_loop(agent) end
    %TTX.Dots{agent_pid: agent}
  end

  @spec update_loop(pid) :: no_return
  defp update_loop(agent) do
    Agent.update(agent, fn v ->
      DotsState.update(v)
    end)
    Process.sleep(300)
    update_loop(agent)
  end

  @spec get_draw_string(t) :: String.t()
  def get_draw_string(dots) do
    dots.agent_pid
    |> Agent.get(fn v ->
      DotsState.compute_text(v)
    end)
  end
end
