defmodule TTX.Components.ArtifactItem do
  alias TTX.Composition.{Dot, Component}
  alias IO.ANSI

  @artifacts [
    "&",
    "$",
    "%",
    "#",
    "*",
    "^",
    ":",
    ";",
    "'",
    "`",
    "\"",
    "@",
    "!",
    "?",
    "/",
    "\\",
    "|"
  ]

  @moduledoc """
  Компонент-артифакт. Простая точка, которая обновляется по особой логике.
  Обновляется синхронно, по приказу "сверху". Из входящих параметров имеет:

  - `:remoteness` -  место нахождения от центра. Относительное число (0..1), где 0 - центр,
  а 1 - максимально далеко от него
  """

  use TypedStruct

  typedstruct enforce: true do
    field(:agent, pid())
    field(:remoteness, float())
  end

  @spec new(%{remoteness: any, x: integer, y: integer}) :: t()
  def new(%{x: x, y: y, remoteness: r}) do
    {:ok, agent} = Agent.start_link(fn -> Dot.new(x, y, gen_artifact(r)) end)

    %__MODULE__{
      agent: agent,
      remoteness: r
    }
  end

  @spec update(t()) :: t()
  def update(%__MODULE__{agent: agent, remoteness: r} = self) do
    Agent.update(agent, fn dot -> Dot.update_val(dot, gen_new_artifact(r, dot.val)) end)
    self
  end

  defp gen_new_artifact(val, old) do
    case gen_artifact(val) do
      nil -> nil
      ^old -> gen_new_artifact(val, old)
      x -> x
    end
  end

  defp gen_artifact(rem_val) do
    rand_val = :rand.uniform() |> :math.pow(5)

    if rand_val > rem_val do
      Enum.random(@artifacts)
      |> colorize_randomly()
    else
      nil
    end
  end

  defp colorize_randomly(text) do
    col = ANSI.color(100 + floor(:rand.uniform() * 155))
    col <> text <> ANSI.reset()
  end

  defimpl Component, for: __MODULE__ do
    def elements(x) do
      [
        Agent.get(x.agent, fn x -> x end)
      ]
    end
  end
end
