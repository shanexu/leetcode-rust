#!/usr/bin/env python3

def solve_sudoku(grid):
    chars = ['+', 'o', 's', 't']

    def is_valid(row, col, c):
        # 检查行是否重复
        if c in grid[row]:
            return False
        # 检查列是否重复
        if c in [grid[i][col] for i in range(4)]:
            return False
        return True

    def backtrack():
        for i in range(4):
            for j in range(4):
                if grid[i][j] == '.':
                    for c in chars:
                        if is_valid(i, j, c):
                            grid[i][j] = c
                            if backtrack():
                                return True
                            grid[i][j] = '.'  # 回溯
                    return False
        return True

    backtrack()
    return grid

res = solve_sudoku([
	['.', 's', '+', '.'],
	['+', '.', '.', '.'],
	['t', '.', '.', '.'],
	['s', '.', '.', '.']])
for l in res:
	print(l)
