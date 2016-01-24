#!/usr/bin/env python
# -*- coding: utf-8 -*-
# 618479
n = int(input())
a = map(int, input().split())
cnt = 0
for num in a:
    while (num % 3) == 2 or (num % 2) == 0:
        num -= 1
        cnt += 1

print(cnt)
