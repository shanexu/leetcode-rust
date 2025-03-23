#!/usr/bin/env python3

import sys


def solve_sudoku(grid):
	chars = ["x", "o", "s", "t"]
	n = len(grid)
	if n == 5:
		chars = ["x", "o", "s", "t", "f"]

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
		board.append([s[i : i + 1] for i in range(len(s))])
	return board


def solve(board_str, i, j):
	res = solve_sudoku(board_strs_to_board(board_str))
	return res[i][j]


def main():
	i = int(sys.argv[1])
	j = int(sys.argv[2])
	print(i, j, sys.argv[3:])
	print(solve(sys.argv[3:], i, j))


main()
