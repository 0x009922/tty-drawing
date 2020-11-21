defmodule TTX.Composition.Builder.Frame do
  use TypedStruct
  alias TTX.Composition.Builder.Frame

  @type section() :: {integer(), integer()}

  typedstruct enforce: true do
    field(:x, integer())
    field(:y, integer())
    field(:w, non_neg_integer())
    field(:h, non_neg_integer())
  end

  @spec create(integer(), integer(), non_neg_integer(), non_neg_integer()) :: t()
  def create(x, y, w, h) do
    %Frame{
      x: x,
      y: y,
      w: w,
      h: h
    }
  end

  @spec from_window(TTX.Composition.Window.t()) :: t()
  def from_window(%TTX.Composition.Window{x: x, y: y, width: w, height: h}) do
    create(x, y, w, h)
  end

  @doc """
  Создание фрейма в абсолютных координатах,
  отталкиваясь от его родительского фрейма
  """
  @spec add_coords(t(), t()) :: t()
  def add_coords(child, parent) do
    create(parent.x + child.x, parent.y + child.y, child.w, child.h)
  end

  @spec intersection(t(), t()) :: t() | nil
  def intersection(a, b) do
    # извлекаю из каждого фрейма отрезки на осях OX/OY
    # в абсолютных координатах, смотрю пересечения и
    # если по обоим есть, собираю из них фрейм

    new_sections =
      [
        &extract_yy/1,
        &extract_xx/1
      ]
      # функции преобразую в отрезки по этим функциям
      |> Enum.map(fn f -> {f.(a), f.(b)} end)
      # аккумулирую пересечения отрезков
      |> Enum.reduce([], fn {sect_a, sect_b}, acc ->
        case acc do
          # если до этого был фейл, то фейл
          nil ->
            nil

          acc ->
            # пересекаю. если будет не ок, то аккумулятор станет nil
            case sections_intersection(sect_a, sect_b) do
              nil -> nil
              inter -> [inter | acc]
            end
        end
      end)

    # теперь получил либо nil, либо список [xx, yy]

    case new_sections do
      nil ->
        nil

      [{x1, x2}, {y1, y2}] ->
        # собираю из отрезков фрейм
        %Frame{
          x: x1,
          w: x2 - x1,
          y: y1,
          h: y2 - y1
        }
    end
  end

  @spec extract_xx(t()) :: section()
  defp extract_xx(%Frame{x: a, w: b}), do: rel_to_abs(a, b)

  @spec extract_yy(t()) :: section()
  defp extract_yy(%Frame{y: a, h: b}), do: rel_to_abs(a, b)

  @spec rel_to_abs(integer(), non_neg_integer()) :: section()
  defp rel_to_abs(origin, offset), do: {origin, origin + offset}

  @spec sections_intersection(section(), section()) :: section() | nil
  defp sections_intersection(a, b)
  defp sections_intersection({a1, a2}, {b1, b2}) when a2 < b1 or b2 < a1, do: nil

  defp sections_intersection({a1, a2}, {b1, b2}) do
    {max(a1, b1), min(a2, b2)}
  end
end
