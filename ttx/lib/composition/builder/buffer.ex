defmodule TTX.Composition.Builder.Buffer do
  @moduledoc """
  Сборник функций для работы с буффером.

  Буффер - это просто карта, `%{}`
  """
  use TypedStruct

  typedstruct enforce: true do
    field(:rows, non_neg_integer())
    field(:columns, non_neg_integer())
    field(:map, map())
  end

  @spec new(non_neg_integer(), non_neg_integer()) :: t()
  def new(rows, columns) do
    %__MODULE__{
      rows: rows,
      columns: columns,
      map: %{}
    }
  end

  @doc """
  Запись в буфер значения на заданную позицию
  """
  @spec write(t(), integer(), integer(), String.t()) :: t()
  def write(buff, x, y, val) do
    key = {x, y}

    if Map.has_key?(buff.map, key) do
      update_buff_map(buff, Map.replace!(buff.map, key, val))
    else
      update_buff_map(buff, Map.put(buff.map, key, val))
    end
  end

  defp update_buff_map(buff, new_val) do
    %__MODULE__{buff | map: new_val}
  end

  @doc """
  Построение конечной строки из буфера
  """
  @spec build(t()) :: Strint.t()
  def build(%__MODULE__{map: buff, rows: rows, columns: columns}) do
    # пока сделаю хоть как, потом оптимизации

    0..(rows - 1)
    |> Enum.reduce("", fn row, acc ->
      builded_line =
        Enum.reduce(0..(columns - 1), "", fn col, acc ->
          # если в буффере ничего нет, то прибавляю пробел
          # иначе само значение
          case Map.get(buff, {col, row}) do
            nil -> acc <> " "
            some_binary -> acc <> some_binary
          end
        end)

      # если аккум пустой, то просто добавляю новую строчку
      # иначе добавляю её с переносом
      case acc do
        "" -> builded_line
        x -> x <> "\n" <> builded_line
      end
    end)
  end
end
