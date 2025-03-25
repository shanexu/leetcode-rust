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
		s = s.replace("*", ".")
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
	board_str = ''.join(board_strs)
	ws = re.compile(r'\s+')
	board_str = ws.sub('', board_str)
	board_str = board_str.replace('*', '.')
	board_str = board_str.replace('_', '.')
	board = None
	if len(board_str) == 16:
		if not bool(re.match(r'^[ostx.?]+$', board_str)):
			print('含有非法字符')
			exit(1)
		else:
			board = [board_str[i] for i in range(16)]
			board = [board[i:i+4] for i in range(0, 16, 4)]
	elif len(board_str) == 25:
		if not bool(re.match(r'^[ostxf.?]+$', board_str)):
			print('含有非法字符')
			exit(1)
		else:
			board = [board_str[i] for i in range(25)]
			board = [board[i:i+5] for i in range(0, 25, 5)]
	else:
		print("矩阵长度不对，既不是16也不是25")
		exit(1)

	print_board(board)
	qs = []
	for i in range(len(board)):
		for j in range(len(board)):
			if board[i][j] == '?':
				board[i][j] = '.'
				qs.append((i, j))
				break
	print(qs)
	print()
	board = solve_sudoku(board)
	for q in qs:
		print(q, '=', board[q[0]][q[1]])
	print_board(board)


main()
