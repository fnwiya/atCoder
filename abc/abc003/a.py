#!/usr/bin/python
# -*- coding: utf-8 -*-
# 600981
n = int(input())
s = 0
for x in range(n):
    s += x + 1
print(s*10000/n)
