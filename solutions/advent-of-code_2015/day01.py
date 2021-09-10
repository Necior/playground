import pytest


def get_floor(instructions: str) -> int:
    return instructions.count("(") - instructions.count(")")


@pytest.mark.parametrize(
    "instructions, floor",
    [
        ("(())", 0),
        ("()()", 0),
        ("(((", 3),
        ("(()(()(", 3),
        ("))(((((", 3),
        ("())", -1),
        ("))(", -1),
        (")))", -3),
        (")())())", -3),
    ],
)
def test_get_floor(instructions: str, floor: int):
    assert floor == get_floor(instructions)


def get_first_basement_visit_position(instructions: str) -> int:
    position = floor = 0
    for i in instructions:
        position += 1
        if i == "(":
            floor += 1
        else:
            floor -= 1
        if floor == -1:
            return position


@pytest.mark.parametrize("instructions, position", [(")", 1), ("()())", 5)])
def test_get_first_basement_visit_position(instructions: str, position: int):
    assert position == get_first_basement_visit_position(instructions)
