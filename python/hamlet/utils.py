import re
from typing import List


def extract_numbers(raw: str) -> List[List[int]]:
    """
    Extract all integers from a multiline string.

    This utility function is useful for extracting numbers from unstructured tables.
    """
    lines = raw.strip().split("\n")
    lines = [l.replace(",", "").replace(".", "") for l in lines]
    regex = re.compile(r"([0-9]+)[^0-9]")

    def get_numbers(line: str) -> List[int]:
        return [int(n) for n in regex.findall(line)]

    return [get_numbers(l) for l in lines]
