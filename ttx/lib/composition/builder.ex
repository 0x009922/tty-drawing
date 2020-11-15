defmodule TTX.Composition.Builder do
  @moduledoc """
  Сборка композиции в готовую строку
  """

  alias TTX.Composition.{Window, Dot, Offset, Component}
  alias TTX.Composition.Builder.{Buffer, Frame}

  @doc """
  Построение конечной строки из композиции на основе количества строчек и столбцов
  """
  @spec build([any], non_neg_integer, non_neg_integer) :: String.t()
  def build(elements, rows, columns) do
    # Начальный фрейм - всё окно отрисовки
    frame = Frame.create(0, 0, columns, rows)

    Buffer.new(rows, columns)
    |> fill_buffer(elements, frame)
    |> Buffer.build()
  end

  @spec fill_buffer(Buffer.t(), list(), Frame.t()) :: Buffer.t()
  defp fill_buffer(buffer, elements, frame) do
    Enum.reduce(elements, buffer, fn elem, buffer ->
      # смотря что за элемент - то и делаю
      case elem do
        %Window{} = window ->
          # окно - значит, обновляем фрейм
          new_frame =
            Frame.create(window.x, window.y, window.width, window.height)
            |> Frame.add_coords(frame)
            |> Frame.intersection(frame)

            fill_buffer(buffer, window.elements, new_frame)

        %Offset{} = offset ->
          # смещение - обновляю фрейм, но не так, как window
          # ширину и длину беру родительскую - всё равно она будет гарантированно её меньше
          new_frame =
            Frame.create(offset.x, offset.y, frame.w, frame.h)
            |> Frame.add_coords(frame)
            |> Frame.intersection(frame)

            fill_buffer(buffer, offset.elements, new_frame)

        %Dot{} = dot ->
          # нашли точку - заполняю буффер!
          # в пределах текущего фрейма, конечно
          insert_dot(buffer, frame, dot)

        component ->
          # Компонент! Динамически отдаст мне элементы
          fill_buffer(buffer, Component.elements(component), frame)
      end
    end)
  end

  @spec insert_dot(Buffer.t(), Frame.t(), Dot.t()) :: Buffer.t()
  defp insert_dot(buffer, frame, dot)

  defp insert_dot(
         buff,
         %Frame{w: w, h: h},
         %Dot{x: x, y: y}
       )
       when x < 0 or
              y < 0 or
              x > w or
              y > h,
       do: buff

  defp insert_dot(buff, _frame, dot) do
    Buffer.write(buff, dot.x, dot.y, dot.val)
  end
end
