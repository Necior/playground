import sys

import pytest


@pytest.mark.parametrize(
    "string, nice",
    [
        ("ugknbfddgicrmopn", True),
        ("aaa", True),
        ("jchzalrnumimnmhp", False),
        ("haegwjzuvuyypxyu", False),
        ("dvszwmarrgswjxmb", False),
    ],
)
def test_is_nice(string: str, nice: bool):
    assert nice == is_nice(string)


def is_nice(string: str) -> bool:
    vowels = "aeiou"
    disallowed = "ab cd pq xy".split()
    at_least_three_vowels = sum(string.count(v) for v in vowels) >= 3
    at_least_one_letter_twice_in_a_row = (
        sum(a == b for a, b in zip(string[:-1], string[1:])) >= 1
    )
    no_disallowed_substrings = all(d not in string for d in disallowed)
    return (
        at_least_three_vowels
        and at_least_one_letter_twice_in_a_row
        and no_disallowed_substrings
    )


@pytest.mark.parametrize(
    "string, nice",
    [
        ("qjhvhtzxzqqjkmpb", True),
        ("xxyxx", True),
        ("uurcxstgmygtbstg", False),
        ("ieodomkazucvgmuy", False),
    ],
)
def test_is_nice2(string: str, nice: bool):
    assert nice == is_nice2(string)


def is_nice2(string: str) -> bool:
    pairs = (a + b for a, b in zip(string[:-1], string[1:]))
    doubled_pair = sum(string.count(p) >= 2 for p in pairs) >= 1
    repeated_letter = sum(a == b for a, b in zip(string[2:], string[:-2])) >= 1
    return doubled_pair and repeated_letter


if __name__ == "__main__":
    nice_count = nice2_count = 0
    for line in sys.stdin:
        nice_count += is_nice(line)
        nice2_count += is_nice2(line)
    print(f"There are {nice_count} nice words")
    print(f"There are {nice2_count} nice2 words")
