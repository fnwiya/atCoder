#!/usr/bin/env python
# -*- coding: utf-8 -*-
# 618142
n = int(input())
a = [int(input()) for _ in range(n)]
ret = list(set(a))
ret.sort()
print(ret[-2])
