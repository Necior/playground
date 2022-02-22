"""
Source: https://www.youtube.com/watch?v=41ON2EghJi0
Title: Searching a Matrix
Description:

Given a matrix that is organized such that the numbers will always be sorted
left to right, and the first number of each row will always be greater then the
last element of the last row (mat[i][0] > mat[i - 1][-1]), search for a
specific value in the matrix and return whether it exists.
"""

from typing import List


def search_matrix(matrix: List[List[int]], value: int) -> bool:
    N = len(matrix)
    low, high = 0, N * N - 1
    while low <= high:
        mid = low + (high - low) // 2
        row, col = divmod(mid, N)
        v = matrix[row][col]
        if v == value:
            return True
        if v < value:
            low = mid + 1
        if v > value:
            high = mid - 1
    return False


def test_search_matrix():
    mat = [
        [1, 3, 5, 8],
        [10, 11, 15, 16],
        [24, 27, 30, 31],
    ]

    assert search_matrix(mat, 4) is False
    assert search_matrix(mat, 10) is True
