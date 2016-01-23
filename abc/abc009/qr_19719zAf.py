#!/usr/bin/env python
# -*- coding: utf-8 -*-
n = int(input())
a = [int(input()) for _ in range(n)]
a = [3, 4, 5]
ret = list(set(a))
ret = ret.sort()
ret = ret.reverse()
print(ret[1])
