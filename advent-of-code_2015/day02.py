import sys

import pytest


def calculate_paper_needed(dimensions: str) -> int:
    d1, d2, d3 = sorted([int(x) for x in dimensions.split("x")])
    return 2 * (d1 * d2 + d1 * d3 + d2 * d3) + d1 * d2


@pytest.mark.parametrize(
    "dimensions, paper", [("2x3x4", 58), ("1x1x10", 43), ("4x23x21", 1402)]
)
def test_calculate_paper_needed(dimensions: str, paper: int):
    assert paper == calculate_paper_needed(dimensions)


def calculate_ribbon_needed(dimensions: str) -> int:
    d1, d2, d3 = sorted([int(x) for x in dimensions.split("x")])
    around = 2 * (d1 + d2)
    bow = d1 * d2 * d3
    return around + bow


@pytest.mark.parametrize("dimensions, ribbon", [("2x3x4", 34), ("1x1x10", 14)])
def test_calculate_ribbon_needed(dimensions: str, ribbon: int):
    assert ribbon == calculate_ribbon_needed(dimensions)


if __name__ == "__main__":
    total_paper = total_ribbon = 0
    line = sys.stdin.readline()
    while line:
        total_paper += calculate_paper_needed(line)
        total_ribbon += calculate_ribbon_needed(line)
        line = sys.stdin.readline()
    print(f"Total paper needed: {total_paper}")
    print(f"Total ribbon needed: {total_ribbon}")
