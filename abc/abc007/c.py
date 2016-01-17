#!/usr/bin/python
# -*- coding: utf-8 -*-
# 613004
R, C = map(int, input().split())
sy, sx = map(lambda x: int(x) - 1, input().split())
gy, gx = map(lambda x: int(x) - 1, input().split())
grid = [input() for i in range(R)]
minMoves = {}
minMoves[(sy, sx)] = 0
que = [sy, sx]
DY = [-1, 1, 0, 0]
DX = [0, 0, -1, 1]

while que:
    y, x = (que.pop(0), que.pop(0))
    for dy, dx in zip(DY, DX):
        ny, nx = (y + dy, x + dx)
        if 0 <= ny < R and 0 <= nx < C and grid[ny][nx] == '.':
            if (ny, nx) not in minMoves:
                minMoves[(ny, nx)] = minMoves[(y, x)] + 1
                que.append(ny)
                que.append(nx)

print(minMoves[(gy, gx)])
