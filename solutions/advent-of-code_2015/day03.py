import sys

import pytest


@pytest.mark.parametrize(
    "moves, visited_count", [(">", 2), ("^>v<", 4), ("^v^v^v^v^v", 2)]
)
def test_count_visited_houses(moves: str, visited_count: int):
    assert visited_count == len(visited_houses(moves))


def visited_houses(moves: str) -> int:
    # Let's represent position as a complex number
    position = 0j
    visited = {position}
    for m in moves:
        if m == "^":
            position += 1j
        elif m == "v":
            position -= 1j
        elif m == ">":
            position += 1
        elif m == "<":
            position -= 1
        visited.add(position)
    return visited


@pytest.mark.parametrize(
    "moves, visited_count", [("^v", 3), ("^>v<", 3), ("^v^v^v^v^v", 11)]
)
def test_count_visited_houses_with_robo_santa(moves: str, visited_count: int):
    assert visited_count == len(visited_houses_with_robo_santa(moves))


def visited_houses_with_robo_santa(moves: str) -> int:
    visited_santa = visited_houses(moves[::2])
    visited_robo_santa = visited_houses(moves[1::2])
    return visited_santa | visited_robo_santa


if __name__ == "__main__":
    line = next(sys.stdin)
    print(len(visited_houses(line)))
    print(len(visited_houses_with_robo_santa(line)))
