from collections import defaultdict
from dataclasses import dataclass
import sys
from typing import Iterable, List, Tuple

import pytest


@dataclass(frozen=True)
class Instruction:
    command: str
    corner1: Tuple[int, int]
    corner2: Tuple[int, int]


@pytest.mark.parametrize(
    "instruction, parsed",
    [
        ("turn on 0,0 through 999,999", Instruction("on", (0, 0), (999, 999))),
        ("toggle 0,0 through 999,0", Instruction("toggle", (0, 0), (999, 0))),
        (
            "turn off 499,499 through 500,500",
            Instruction("off", (499, 499), (500, 500)),
        ),
    ],
)
def test_parse(instruction: str, parsed: Instruction):
    assert parsed == parse(instruction)


def parse(instruction: str) -> Instruction:
    """Poor man's parser."""

    if " off " in instruction:
        command = "off"
    elif " on " in instruction:
        command = "on"
    else:
        command = "toggle"
    rem = instruction.split()[1:]
    if command != "toggle":
        rem = rem[1:]
    pair1, _, pair2 = rem
    corner1 = tuple(int(n) for n in pair1.split(","))
    corner2 = tuple(int(n) for n in pair2.split(","))
    return Instruction(command, corner1, corner2)


@pytest.mark.parametrize(
    "corner1, corner2, positions",
    [
        (
            (0, 0),
            (1, 3),
            [(0, 0), (0, 1), (0, 2), (0, 3), (1, 0), (1, 1), (1, 2), (1, 3)],
        )
    ],
)
def test_get_positions(
    corner1: Tuple[int, int],
    corner2: Tuple[int, int],
    positions: List[Tuple[int, int]],
) -> Iterable[Tuple[int, int]]:
    assert sorted(positions) == sorted(get_positions(corner1, corner2))


def get_positions(
    corner1: Tuple[int, int], corner2: Tuple[int, int]
) -> Iterable[Tuple[int, int]]:
    min_row = min(corner1[0], corner2[0])
    max_row = max(corner1[0], corner2[0])
    min_col = min(corner1[1], corner2[1])
    max_col = max(corner1[1], corner2[1])
    for row in range(min_row, max_row + 1):
        for col in range(min_col, max_col + 1):
            yield (row, col)


def simulate():
    grid = defaultdict(lambda: defaultdict(bool))
    for line in sys.stdin:
        instruction = parse(line)
        positions = get_positions(instruction.corner1, instruction.corner2)
        if instruction.command == "on":
            for row, col in positions:
                grid[row][col] = True
        elif instruction.command == "off":
            for row, col in positions:
                grid[row][col] = False
        elif instruction.command == "toggle":
            for row, col in positions:
                grid[row][col] = not grid[row][col]
        else:
            raise ValueError("Unexpected command")
    counter = 0
    for row in grid.values():
        for light in row.values():
            counter += light
    print(f"{counter} lights turned on")


def simulate2():
    """Adapted from ``simulate``."""

    grid = defaultdict(lambda: defaultdict(int))
    for line in sys.stdin:
        instruction = parse(line)
        positions = get_positions(instruction.corner1, instruction.corner2)
        if instruction.command == "on":
            for row, col in positions:
                grid[row][col] += 1
        elif instruction.command == "off":
            for row, col in positions:
                if grid[row][col] > 0:
                    grid[row][col] -= 1
        elif instruction.command == "toggle":
            for row, col in positions:
                grid[row][col] += 2
        else:
            raise ValueError("Unexpected command")
    counter = 0
    for row in grid.values():
        for light in row.values():
            counter += light
    print(f"{counter} is the total brightness")


if __name__ == "__main__":
    # simulate()
    simulate2()
