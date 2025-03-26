#!/usr/bin/env python3

import sys
import re


def solve_sudoku(grid, chars):
    n = len(grid)
    if not chars:
        chars = [str(i + 1) for i in range(n)]

    def is_valid(row, col, c):
        # 检查行是否重复
        if c in grid[row]:
            return False
        # 检查列是否重复
        if c in [grid[i][col] for i in range(n)]:
            return False
        return True

    def backtrack():
        for i in range(n):
            for j in range(n):
                if grid[i][j] == ".":
                    for c in chars:
                        if is_valid(i, j, c):
                            grid[i][j] = c
                            if backtrack():
                                return True
                            grid[i][j] = "."  # 回溯
                    return False
        return True

    backtrack()
    return grid


def board_strs_to_board(strs):
    board = []
    for s in strs:
        s = s.replace(" ", "")
        s = s.replace("_", ".")
        s = s.replace("*", ".")
        board.append([s[i : i + 1] for i in range(len(s))])
    return board


def solve(board_str, i, j):
    res = solve_sudoku(board_strs_to_board(board_str))
    return res[i][j]


def print_board(board):
    for line in board:
        print(" ".join(line))


sq = {1: 1, 4: 2, 9: 3, 16: 4, 25: 5, 36: 6, 49: 7, 64: 8, 81: 9}


def main():
    board_strs = sys.argv[1:]
    board_str = "".join(board_strs)
    ws = re.compile(r"\s+")
    board_str = ws.sub("", board_str)
    board_str = board_str.replace("*", ".")
    board_str = board_str.replace("_", ".")
    board = None
    n = len(board_str)
    r = sq.get(n)
    if r is None:
        print("长度不对")
        exit(1)
    chars = None
    if n == 16:
        if re.match(r"^[ostx.?]+$", board_str):
            chars = ["o", "x", "t", "s"]
    if n == 25:
        if re.match(r"^[ostxf.?]+$", board_str):
            chars = ["o", "x", "t", "s", "f"]
    if chars is None:
        p = re.compile("^[" + "".join([str(i + 1) for i in range(r)]) + ".?]+$")
        if not p.match(board_str):
            print("输入字符有误")
            exit(1)
    board = [board_str[i] for i in range(n)]
    board = [board[i : i + r] for i in range(0, n, r)]

    print_board(board)
    qs = []
    for i in range(len(board)):
        for j in range(len(board)):
            if board[i][j] == "?":
                board[i][j] = "."
                qs.append((i, j))
                break
    print(qs)
    print()
    board = solve_sudoku(board, chars)
    for q in qs:
        print(q, "=", board[q[0]][q[1]])
    print_board(board)


if __name__ == "__main__":
    main()
