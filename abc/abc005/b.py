#!/usr/bin/python
# -*- coding: utf-8 -*-
# 601841
n = int(input())
t1 = int(input())
mini = t1
for i in range(n-1):
    t = int(input())
    mini = min(mini, t)
print(mini)
