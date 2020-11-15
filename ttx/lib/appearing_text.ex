defmodule TTX.AppearingText do
  @moduledoc """
  Управляет асинхронным появлением текста. Появляется сам по себе. потихоньку со временем.
  Когда текст полностью появляется, он вызывает некий изначально переданный колбек, говорящий
  о том, что всё, можно удалять всю инфу об этом модуле, он своё отработал.
  В то же время ликвидируется агент с состоянием. После этого любые опросы будут недействительными.
  """

  use TypedStruct

  alias TTX.AppearingText, as: SyncState

  typedstruct enforce: true do
    field(:agent, pid())
    field(:exit_fn, fun())
  end

  defmodule State do
    typedstruct enforce: true do
      field(:source, String.t())
      field(:full_len, pos_integer())
      field(:curr_len, pos_integer(), default: 0)
      field(:curr_slice, String.t(), default: "")
    end

    @spec init(String.t()) :: t()
    def init(source) do
      %State{
        source: source,
        full_len: String.length(source)
      }
    end

    @spec tick(t) :: t | :done
    def tick(%State{curr_len: x, full_len: limit}) when x >= limit, do: :done

    def tick(state) do
      len = state.curr_len + 1
      slice = String.slice(state.source, 0..(len - 1))

      Map.merge(state, %{
        curr_len: len,
        curr_slice: slice
      })
    end

    @spec current_slice(t()) :: String.t()
    def current_slice(%State{curr_slice: x}), do: x
  end

  @spec init(Strint.t(), fun()) :: t()
  def init(text, exit_fn) do
    {:ok, agent} = Agent.start_link(fn -> State.init(text) end)
    sync = %SyncState{agent: agent, exit_fn: exit_fn}
    spawn_link(fn -> update_loop(sync) end)
    sync
  end

  def current_text(%SyncState{agent: agent}) do
    if Process.alive?(agent) do
      agent
      |> Agent.get(fn state ->
        state
        |> State.current_slice()
      end)
    else
      :done
    end
  end

  defp update_loop(sync_state) do
    Process.sleep(100)

    %{agent: agent_pid} = sync_state

    update_result =
      Agent.get_and_update(agent_pid, fn state ->
        case State.tick(state) do
          :done -> {:done, state}
          updated -> {:alive, updated}
        end
      end)

    case update_result do
      :done ->
        Process.sleep(1000)
        Agent.stop(agent_pid)
        sync_state.exit_fn.()

      :alive ->
        update_loop(sync_state)
    end
  end
end
