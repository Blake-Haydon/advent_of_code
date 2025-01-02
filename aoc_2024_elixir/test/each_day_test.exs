defmodule EachDayTest do
  use ExUnit.Case

  test "d01p1" do
    assert Day01.part1() == 1_941_353
  end

  test "d01p2" do
    assert Day01.part2() == 22_539_317
  end

  test "d02p1" do
    assert Day02.part1() == 321
  end

  test "d02p2" do
    assert Day02.part2() == 386
  end
end
