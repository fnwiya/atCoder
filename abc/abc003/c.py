#!/usr/bin/python
# -*- coding: utf-8 -*-
# 601006
n, k = map(int, input().split())
r = list(map(int, input().split()))
r.sort()
r.reverse()
rate = 0
for i in range(k):
    rate += r[i]/2**(i+1)

print(rate)
