#!/usr/bin/python
# -*- coding: utf-8 -*-
# 602485
n = int(input())
a, b, c = 0, 0, 1
if n < 3:
    c = 0
else:
    for i in range(n - 3):
        a, b, c = b, c, (a + b + c) % 10007
print(c % 10007)
