# TTX

**TODO: Add description**

## Что делаю

Повторю, хотя бы частично, то, что есть на js. Есть некая рамка где-то в каком-то месте терминала, внутри неё спавнятся и пропадают слова.

Какими итерациями это делать: сначала модель и композиция, затем отрисовка (в самом конце, чисто алгоритмы уже).

Композиция - это структура из окон/смещений/изображений, которая в конце концов будет отрисовываться. Эта композиция может когда  угодно меняться. Я создаю некий объект, который, например, занимается тем, что как-то по-особенному отрисовывает текст, который я ему даю исходно. Я понятия не имею, как он там изнутри работает, но он может обновлять своё состояние и свою композицию. Что я от него получаю? Какую-то композицию, которая при этом может всегда обновиться. А ещё я могу захотеть получить от него назад какие-то колбэки, или передать ему внутрь. Если я, например, хочу этот объект уничтожить, или я хочу ему передать функцию, которую он будет изнутри вызывать? Какая модель обновления данных? Шина, поток, циркуляция данных, сообщения процессов друг другу.

Возможно, отрисовку надо делать каскадно, рекурсивно. Не из какой-то уже готовой структуры, которая временами изменяется (в ФП ничего не изменяется). Нет, мы эмитим изменения в данных в некий общий слушатель, который раз в 50 мс (например) запускает перерисовку в случае, если было хоть одно изменение. Как запускает? Проходит каскадно по каким-то объектам, которые реализуют некий протокол отрисовки. Протокол специально будет устроен рекурсивно, фрактально, в глубину, перерисовывая и перерисовывая.

К слову, можно и не слушать какие-то там изменения, а просто перерисовывать каждые 50мс и всё.

Другой вопрос. Если у нас всё state-less и неизменяемое, то как создавать какие-либо "объекты", которые доступны для чтения извне, но при этом могут как угодно изменяться изнутри, незаметно для наружного наблюдателя? Всё ИММУТАБЕЛЬНО, такого быть не может. Допустим, можно запускать специальную функцию, которая создаст специальную структуру данных и передаст её в отдельный процесс, который будет как угодно лочиться и обновлять эту структуру в самой себе. НО! Окей, структура-то обновляется, но как извне её получить при отрисовке?

Тогда возможен старый добрый вариант с "тиканьем" всех элементов экспозиции синхронно. Типа так:

```elixir
def compose(item) do
  # тикаю, обновляю состояние объекта и всех потомков
  item.tick()

  # беру композицию - множество window, offset, image
  composition = item.compose()

  # отрисовываю каждый
  composition
  |> Enum.each(fn item -> do
    case item do
      # окно - ок, рисую композицию окна внутри него
      # смещение - изменяю текущий фрейм, рисую композицию внутри
      # изображение - ок, просто рисую изображение, это конечный элемент
      # нечто, реализующее протокол compose - делаю compose(item)
    end
  end)
end
```

#### Асинхронный вариант

Есть, допустим, некий лупер:

```elixir
defmodule Looper do
  def start do
    loop(state?)
  end

  defp loop(state) do
    # await 50ms
    Tick.tick(state)
    loop(state)
  end
end

defprotocol Tick do
  def tick(val)
end
```

В него я передаю тикающий объект. Он будет запускать рекурсивную отрисовку. Дальше, когда я хочу использовать некое состояние, обновляющееся асинхронно, я делаю это так:

```elixir
defmodule AnimatedText do
  defstruct %{}

  def create do
    # создаю агента, в него кладу состояние
    # создаю лупер, который будет с какой-то своей логикой обновлять это состояние. Передаю в него pid агента
    # ?????
    # Теперь у меня лупер будет асинхронно обновлять состояние у агента

    # возвращаю некую структуру со всеми данными, а именно с pid агента, состоянием. Или чем угодно ещё
  end

  def get_image(val) do
    # беру из val pid агента, изымаю из него состояние, беру из него изображение, отдаю

    # или у меня в этом состоянии могут быть ещё некие элементы, составляющие композицию. Отдаю их, беру из них
    # так же рекурсивно их композицию
  end
end
```

С таким подходом есть опасение, что это плохо скажется на производительности - ведь для одного обхода всего дерева композиции нужно будет последовательно опросить состояние у каждого агента из древа.

Ещё есть вопрос - обновлять ли сами изображение и те структуры, которые прямо пойдут в рендер, внутри лупера, вручную следя за изменениями, или каждый раз рассчитывать всё при составлении самой композиции? Думаю, первый вариант более оптимален. К тому же перерасчёт так будет проходить в отдельном потоке, процессе, и не будет блочить основной поток. В основном потоке надо быстренько пробежаться по агентам и забрать у них всё готовое.

Вообще получается целая компонентная система, без порталов. Строго одно в другое. Можно даже передавать какие-то узлы в другие узлы как в слоты. Получаются целые рендер-функции... `VNode`!

#### 1. 
