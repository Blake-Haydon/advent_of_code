defmodule Day01 do
  def load_input() do
    # read file
    # {:ok, text} = File.read("input/01_toy_input.txt")
    {:ok, text} = File.read("input/01_input.txt")

    nums =
      text
      |> String.split("\n")
      |> Enum.flat_map(&String.split(&1, "   "))
      |> Enum.filter(&(&1 != ""))
      |> Enum.map(&String.to_integer/1)

    # take every `n` elements, starting at index `i` from `list`
    take = fn list, n, i ->
      list
      |> Enum.with_index()
      |> Enum.filter(fn {_num, index} -> rem(index, n) == i end)
      |> Enum.map(fn {num, _index} -> num end)
    end

    # created sorted lists from the two inital lists
    list_0 = take.(nums, 2, 0)
    list_1 = take.(nums, 2, 1)

    {list_0, list_1}
  end

  def part1() do
    {list_0, list_1} = load_input()
    list_0_sort = Enum.sort(list_0)
    list_1_sort = Enum.sort(list_1)

    # calculate the difference between the two sorted lists and return the sum
    Enum.zip(list_0_sort, list_1_sort)
    |> Enum.map(fn {a, b} -> abs(a - b) end)
    |> Enum.sum()
  end

  def part2() do
    {list_0, list_1} = load_input()

    # detemine the frequency of each element in the lists
    list_0_freq = Enum.frequencies(list_0)
    list_1_freq = Enum.frequencies(list_1)

    # if the key is not in the second list, default to 0
    list_0_freq
    |> Enum.map(fn {k, v} -> k * v * (list_1_freq[k] || 0) end)
    |> Enum.sum()
  end
end

IO.puts("part 1: #{Day01.part1()}")
IO.puts("part 2: #{Day01.part2()}")
