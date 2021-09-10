import hashlib

import pytest


@pytest.mark.parametrize(
    "secret, answer", [("abcdef", 609043), ("pqrstuv", 1048970)]
)
def test_mine(secret: str, answer: int):
    assert answer == mine(secret, 5)


def mine(secret: str, difficulty: int) -> int:
    answer = 0
    while True:
        answer += 1
        if (
            hashlib.md5((secret + str(answer)).encode()).hexdigest()[
                :difficulty
            ]
            == "0" * difficulty
        ):
            return answer


if __name__ == "__main__":
    secret = input("Provide puzzle input> ")
    print(mine(secret, 5))
    print(mine(secret, 6))
