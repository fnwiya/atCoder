#!/usr/bin/env python
# -*- coding: utf-8 -*-
n = int(input())
a = [int(input()) for _ in range(n)]
a = [6
, 100
, 100
, 100
, 200
, 200
, 200]
ret = set(a).sort(reverse=True)
print(ret[1])
