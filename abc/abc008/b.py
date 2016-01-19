#!/usr/bin/env python
# -*- coding: utf-8 -*-
# 614091
n = int(input())
dic = {}
for i in range(n):
    s = input()
    if s in dic:
        dic[s] += 1
    else:
        dic[s] = 0
print(max(dic.items(), key=lambda x: x[1])[0])
