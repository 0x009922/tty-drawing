defprotocol TTX.Composition.Component do
  @doc """
  Функция принимает на вход сам компонент, чем бы он
  ни был, и должна вернуть список элементов
  """
  @spec elements(any()) :: list()
  def elements(component)
end
