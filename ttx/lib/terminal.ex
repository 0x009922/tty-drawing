defmodule TTX.Terminal do
  @moduledoc """
  Функции для работы с терминалом
  """

  @type size() :: {pos_integer(), pos_integer()}

  @doc """
  Получение размеров терминала
  """
  @spec size() :: size()
  def size() do
    {:ok, cols} = :io.columns()
    {:ok, rows} = :io.rows()
    {rows, cols}
  end
end
