defmodule TTX.AppearingText.State.Test do
  use ExUnit.Case, async: true
  alias TTX.AppearingText.State

  test "initial slice is empty" do
    assert State.init("Oooops") |> State.current_slice() == ""
  end

  test "after 1 tick appear 1 symbol" do
    val =
      State.init("umm? nyah")
      |> State.tick()
      |> State.current_slice()

    assert val == "u"
  end

  test "10 ticks = 10 symbols" do
    text = "abcdefghijklmnopqrstuv"

    state = Enum.reduce(1..10, State.init(text), fn _x, acc -> State.tick(acc) end)

    assert State.current_slice(state) == "abcdefghij"
  end

  test "3-len string appeared after 3 ticks, not 2" do
    state =
      State.init("abc")
      |> State.tick()
      |> State.tick()

    assert State.current_slice(state) == "ab"

    state = State.tick(state)

    assert State.current_slice(state) == "abc"
  end
end
