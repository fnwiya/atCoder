#!/usr/bin/python
# -*- coding: utf-8 -*-
# 601412
board = [input() for i in range(4)]
for l in reversed(board):
    print("".join(reversed(l)))
