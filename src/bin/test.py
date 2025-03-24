#!/usr/bin/env python3

import sys
import re


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

def print_board(board):
	for line in board:
		print(' '.join(line))


def main():
	board_strs = sys.argv[1:]
	if len(board_strs) == 1:
		ws = re.compile(r'\s+')
		board_strs = ws.split(board_strs[0])
	board = board_strs_to_board(board_strs)

	q = None
	for i in range(len(board)):
		for j in range(len(board)):
			if board[i][j] == '?':
				board[i][j] = '.'
				q = (i, j)
				break
		if q != None:
			break
	print(q)
	print_board(board)
	print()
	board = solve_sudoku(board)
	print(q, '=', board[q[0]][q[1]])
	print_board(board)


main()
