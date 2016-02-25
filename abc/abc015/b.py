#!/usr/bin/env python
# -*- coding: utf-8 -*-
import math

n = int(input())
a = input().split()
total = 0
count = 0
for i in range(len(a)):
    if int(a[i]) != 0:
        total += int(a[i])
        count += 1
print(math.ceil(total/count))
