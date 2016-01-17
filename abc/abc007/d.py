#!/usr/bin/python
# -*- coding: utf-8 -*-
a, b = map(int, input().split())
four_cnt = 0
for i in range(a, b + 1):
    s = str(i)
    if s.find("4") or s.find("9"):
        four_cnt += 1
print(four_cnt)
