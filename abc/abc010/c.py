#!/usr/bin/env python
# -*- coding: utf-8 -*-
# 618483
import math

txa, tya, txb, tyb, t, v = map(int, input().split())
n = int(input())
for _ in range(n):
    x, y = map(int, input().split())
    to_distance = math.sqrt((x - txa) ** 2 + (y - tya) ** 2)
    from_distance = math.sqrt((txb - x) ** 2 + (tyb - y) ** 2)
    total_distance = to_distance + from_distance
    if total_distance / v <= t:
        print('YES')
        exit()
print('NO')
