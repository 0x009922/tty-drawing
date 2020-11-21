defmodule TTX.Tools.Ease do
  @moduledoc """
  Абстракция над easing.
  """

  @type timing_function() :: (float() -> float())
  @type tick_function() :: (term, float() -> term)
  @type loop_state() :: %{
    tick_state: any(),
    delta: pos_integer(),
    time: pos_integer(),
    duration: pos_integer(),
    timing_fn: timing_function(),
    tick_fn: tick_function()
  }

  # use TypedStruct

  # typedstruct enforce: true do
  #   field :duration, pos_integer()
  #   field :
  # end

  @spec run(pos_integer(), tick_function(), keyword) :: {:ok, pid}
  def run(duration, tick_fn, opts \\ []) do
    state = Keyword.get(opts, :state, nil)
    tps = Keyword.get(opts, :ticks_per_second, 10)
    timing_fn = Keyword.get(opts, :timing_fn, &default_timing_fn/1)

    pid = spawn_link fn ->
      delta = div 1000, tps

      loop(%{
        tick_state: state,
        duration: duration,
        time: 0,
        delta: delta,
        timing_fn: timing_fn,
        tick_fn: tick_fn
      })
    end

    {:ok, pid}

    # взять начало, рассчитать когда конец
    # в самом начале тикнуть на 0
    # в самом конце на 1
    # затем начать тикать нормально
    # переждать дельту, рассчитать тайминг, вызвать функцию tick_fn
    # после этого замеряю, сколько прошло с последнего сна
    # если оказалось больше последнего тика, выкидываю ошибку с данными (потом подумать и разобраться)
    # если меньше, то сплю дельту минус то, сколько прошло
    # заново
  end

  defp default_timing_fn(val), do: val


  @spec loop(loop_state()) :: no_return()
  defp loop(state)
  # defp loop(%{time: 0} = state) do
  #   # начало
  #   state = fire_tick(state)
  #   Process.sleep(state.delta)
  #   loop()
  # end
  defp loop(%{time: t, duration: d} = state) when t >= d do
    # конец
    fire_tick(state)
  end
  defp loop(state) do
    tick_start = get_now()

    state = fire_tick(state)

    tick_time = get_now() - tick_start

    if tick_time >= state.delta do
      raise "Ooops, tick time is too much!"
    end

    sleep_time = state.delta - tick_time
    Process.sleep(sleep_time)

    # инкремент времени и поехали дальше
    %{state | time: state.time + state.delta}
    |> loop()
  end

  defp get_now(), do: :os.system_time(:millisecond)

  @spec calc_timing(loop_state()) :: float()
  defp calc_timing(%{time: t, duration: d, timing_fn: tfn}) do
    # a = if t <= 0 do
    #   0
    # else
    #   if t >= d, do: d, else: t
    # end

    normalized = case {t, d} do
      {t, _d} when t <= 0 -> 0
      {t, d} when t >= d -> d
      {t, _d} -> t
    end

    tfn.(normalized / d)
  end

  # defp normalize_time(time, duration)
  # defp normalize_time(t, _d) when t <= 0, do: 0
  # defp normalize_time(t, d) when t >= d, do: d
  # defp normalize_time(t, _d), do: t

  @spec fire_tick(loop_state()) :: loop_state()
  defp fire_tick(state) do
    curr_time = calc_timing(state)
    tick_state = state.tick_fn.(state.tick_state, curr_time)
    %{state | tick_state: tick_state}
  end
end
